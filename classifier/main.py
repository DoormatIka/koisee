
import sys
import os
import threading

from uvicorn import run

from collections.abc import Collection
import multiprocessing
from pathlib import Path
from fastapi import FastAPI
from pydantic import BaseModel

from src.finders import ImagePair
from src.wrappers import MethodAction, scan_from_directory

app = FastAPI()

class ScanInput(BaseModel):
    dir: str

class ScanError(BaseModel):
    error: str

class ScanItem(BaseModel):
    paths: tuple[Path, Path]
    similarity: float

class ScanResult(BaseModel):
    matched_images: list[ScanItem]

def convert_image_pair_to_scan_result(pairs: Collection[ImagePair]) -> list[ScanItem]:
    scan_results: list[ScanItem] = []
    for img1, img2 in pairs:
        item = ScanItem(
            paths=(img1.path, img2.path),
            similarity=abs(img1.hash - img2.hash)
        )
        scan_results.append(item)

    return scan_results


@app.post("/scan")
async def scan(item: ScanInput) -> ScanResult | ScanError:
    try:
        dir = Path(item.dir)
        if not dir.is_dir():
            raise FileNotFoundError(dir)
        res = await scan_from_directory(directory=Path(item.dir), choice=MethodAction.HAMMING)
        return ScanResult(matched_images=convert_image_pair_to_scan_result(res))
    except (FileNotFoundError) as e:
        return ScanError(error=f"Directory \"{e}\" not found!")
    except (Exception) as e:
        return ScanError(error=str(e))

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



