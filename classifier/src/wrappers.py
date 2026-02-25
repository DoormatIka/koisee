
from pathlib import Path

from src.infra.logger import Logger
from collections.abc import Collection

from src.finders import HammingClustererFinder, ImageList
from src.hashers import ImageHasher

from typing import override
from enum import Enum



class MethodAction(Enum):
    BRUTE = "brute"
    HAMMING = "hamming"
    @override
    def __str__(self) -> str:
        return self.value


async def clusterer(directory: Path):
    log = Logger()
    imghasher = ImageHasher(size=16)
    bf = HammingClustererFinder(hasher=imghasher, logger=log)

    hashes = await bf.create_hashes_from_directory(directory)
    similar_images = bf.get_similar_objects(hashes)

    return similar_images

async def scan_from_directory(directory: Path, choice: MethodAction) -> Collection[ImageList]: # prototype
    print(f"[START] - Parsing through {directory}")
    return await clusterer(directory)
