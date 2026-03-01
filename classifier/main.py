
from dataclasses import dataclass
import sys
import os
import threading
import asyncio
from typing import Literal
from uuid import uuid4

from uvicorn import run
from contextlib import asynccontextmanager

from collections.abc import Collection
import multiprocessing
from pathlib import Path
from fastapi import FastAPI
from pydantic import BaseModel

from src.finders.types import ImageList
from src.infra.logger import Error, Info, Logger, Warn
from src.hashers import ImageHasher
from src.finders import HammingClustererFinder

class ScanInput(BaseModel):
    dir: str


class ImageData(BaseModel):
    path: Path
    width: int
    height: int
    similarity: int

class ScanItem(BaseModel):
    uuid: str
    paths: list[ImageData]

class ScanResult(BaseModel):
    status: Literal["result"] = "result"
    matched_images: list[ScanItem]
class ScanError(BaseModel):
    status: Literal["error"] = "error"
    error: str
class ScanInProgress(BaseModel):
    status: Literal["progress"] = "progress"
class ScanNoneFound(BaseModel):
    status: Literal["none"] = "none"

ScanIntermediateResult = ScanResult | ScanError | ScanNoneFound | ScanInProgress


def convert_image_pair_to_scan_result(img_list: Collection[ImageList]) -> list[ScanItem]:
    scan_results: list[ScanItem] = []
    for img in img_list:
        im: list[ImageData] = []
        for i in img:
            similarity = img[0].hash - i.hash
            data = ImageData(path=i.path, height=i.height, width=i.width, similarity=similarity)
            im.append(data)
        item = ScanItem(
            uuid=str(uuid4()),
            paths=im,
        )
        scan_results.append(item)

    return scan_results


@dataclass
class Job:
    id: str
    directory: str

job_queue: asyncio.Queue[Job]

job_results: dict[str, ScanIntermediateResult] = dict()
job_res_lock: asyncio.Lock
log: Logger
imghasher: ImageHasher


async def scan(dir: Path) -> ScanResult | ScanError:
    try:
        if not dir.is_dir():
            raise FileNotFoundError(dir)

        bf = HammingClustererFinder(hasher=imghasher, logger=log)

        hashes = await bf.create_hashes_from_directory(Path(dir))
        similar_images = bf.get_similar_objects(hashes)
        return ScanResult(matched_images=convert_image_pair_to_scan_result(similar_images))
    except (FileNotFoundError) as e:
        return ScanError(error=f"Directory \"{e}\" not found!")
    except (Exception) as e:
        return ScanError(error=str(e))

async def worker(): # works on the queue.
    while True:
        item = await job_queue.get()
        try:
            print(f"Processing task: {item}")
            async with job_res_lock:
                job_results[item.id] = ScanInProgress()
            scan_res = await scan(Path(item.directory))
            async with job_res_lock:
                job_results[item.id] = scan_res
        finally:
            print(f"task done.")
            job_queue.task_done()

@asynccontextmanager
async def lifespan(_app: FastAPI): # bridge to queue
    # everything, including the heartbeat route, stops to work on this worker() function.
    # asyncio is probably not the best for this.
    # spawn a completely different process instead?
    global job_queue, job_res_lock, log, imghasher
    job_queue = asyncio.Queue()
    job_res_lock = asyncio.Lock()
    log = Logger()
    imghasher = ImageHasher()

    log.subscribe(Info, print)
    log.subscribe(Warn, print)
    log.subscribe(Error, print)

    worker_task = asyncio.create_task(worker())

    yield

    _ = worker_task.cancel()
    try:
        await worker_task
    except asyncio.CancelledError:
        pass


app = FastAPI(lifespan=lifespan)

@app.post("/scan")
async def queue_dedup(item: ScanInput) -> str:
    job_id = str(uuid4()).strip()
    await job_queue.put(Job(id=job_id, directory=item.dir))

    return job_id

@app.get("/scan/{uuid}")
async def get_scan(uuid: str) -> ScanIntermediateResult:
    async with job_res_lock:
        res = job_results.get(uuid)
        if res is None:
            return ScanNoneFound()
        return res

@app.get("/heartbeat")
async def heartbeat():
    return { "halozy": "heart of night" };


def watch_stdin():
    _ = sys.stdin.read() # if the thread gets past this code, it means the parent died
    os._exit(0)

if __name__ == "__main__":
    with open("env_dump.txt", "w") as f:
        f.write(f"cwd: {os.getcwd()}\n")
        f.write(f"argv: {sys.argv}\n")
        for k, v in os.environ.items():
            f.write(f"{k}={v}\n")

    threading.Thread(target=watch_stdin, daemon=True).start()

    multiprocessing.set_start_method("spawn", force=True)
    multiprocessing.freeze_support()

    run("main:app", port=8080, reload=False)



