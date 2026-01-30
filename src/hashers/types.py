
from pathlib import Path
from dataclasses import dataclass

import imagehash

@dataclass
class CombinedImageHash:
    path: Path
    cropped_hash: imagehash.ImageMultiHash
    hash: imagehash.ImageHash
    pixel_count: int

type ImageHashResult = tuple[CombinedImageHash, None] | tuple[None, str]
