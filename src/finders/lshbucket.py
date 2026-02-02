
import os
import random
import numpy as np

from pathlib import Path
from concurrent.futures import ProcessPoolExecutor, Future, as_completed
from typing import Generic, TypeVar

from hashers.types import CombinedImageHash, ImageHashResult
from hashers.image import imagehash_to_int, ImageHasher

from finders.types import ImagePair
from finders.helpers import is_similar_image, get_supported_extensions

T = TypeVar('T')
class LSHBucket(Generic[T]):
    key_indexes: list[int]
    bucket: list[T] = []
    def __init__(self, key_indexes: list[int]):
        self.key_indexes = key_indexes
    def get_key_similarity(self, bin_val: list[bool]) -> int:
        """
            val should look like [True, False, True, False, False]
        """
        similarity = 0
        for index in self.key_indexes:
            if bin_val[index] == True:
                similarity += 1
        return similarity

def create_random_key_index() -> list[int]:
    resolution = 16
    return [random.randint(0, 63) for _ in range(resolution)]

class LSHBucketFinder[T]():
    hasher: ImageHasher
    buckets: list[LSHBucket[T]]
    def __init__(self, hasher: ImageHasher, resolution: int = 8):
        self.buckets = self._create_buckets_(resolution=resolution)
        self.hasher = hasher

    def _create_buckets_(self, resolution: int = 8):
        buckets: list[LSHBucket[T]] = list()
        lshbucket = LSHBucket[T](key_indexes=create_random_key_index())
        for _ in range(0, resolution):
            buckets.append(lshbucket)
        return buckets

    async def create_hashes_from_directory(self, directory: Path):
        exts = get_supported_extensions()

        n_thread = os.cpu_count()
        if n_thread == None:
            raise ValueError("OS cpu count cannot be found!")

        n_thread = max(n_thread - 2, 2)
        n_images = 0

        for ext in exts:
            for image_path in Path(directory).rglob(f"*{ext}"):
                pass

    def get_similar_objects(self, image_hashes: None):
        pass
