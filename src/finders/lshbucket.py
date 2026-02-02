
import os

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
    def get_key_similarity(self, bin_val: list[int]) -> int:
        """
            val should look like [0, 1, 0, 1, 1, 1, 0, ...]
        """
        similarity = 0
        for index in self.key_indexes:
            if bin_val[index] >= 1:
                similarity += 1
        return similarity

class LSHBucketFinder():
    hasher: ImageHasher
    def __init__(self, hasher: ImageHasher):
        self.hasher = hasher

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
