

from pathlib import Path
from typing import Protocol, TypeVar, Generic
from hashers.types import CombinedImageHash

type ImagePair = tuple[CombinedImageHash, CombinedImageHash]

A = TypeVar('A')
Objects = TypeVar('Objects')
class FinderInterface(Protocol, Generic[A, Objects]):
    def create_hashes_from_directory(self, directory: Path) -> list[A]: ...
    def get_similar_objects(self, image_hashes: list[A]) -> list[Objects]: ...
