
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

from src.finders import ImagePair
from src.wrappers import MethodAction, scan_from_directory

class ScanInput(BaseModel):
    dir: str

class ScanItem(BaseModel):
    paths: tuple[Path, Path]
    similarity: float

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


def convert_image_pair_to_scan_result(pairs: Collection[ImagePair]) -> list[ScanItem]:
    scan_results: list[ScanItem] = []
    for img1, img2 in pairs:
        item = ScanItem(
            paths=(img1.path, img2.path),
            similarity=abs(img1.hash - img2.hash)
        )
        scan_results.append(item)

    return scan_results


@dataclass
class Job:
    id: str
    directory: str

job_queue = asyncio.Queue[Job]()

job_results: dict[str, ScanIntermediateResult] = dict()
job_res_lock = asyncio.Lock()


def scan(dir: Path) -> ScanResult | ScanError:
    try:
        if not dir.is_dir():
            raise FileNotFoundError(dir)
        res = asyncio.run(scan_from_directory(directory=Path(dir), choice=MethodAction.HAMMING))
        return ScanResult(matched_images=convert_image_pair_to_scan_result(res))
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
            scan_res = await asyncio.to_thread(scan, Path(item.directory))
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
    threading.Thread(target=watch_stdin, daemon=True).start()

    multiprocessing.set_start_method("spawn", force=True)
    multiprocessing.freeze_support()

    run("main:app", port=8080, reload=False)



