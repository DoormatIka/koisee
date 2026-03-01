
import numpy as np
import imagehash

from PIL import Image, ImageFile, UnidentifiedImageError
from pathlib import Path

from src.hashers.types import CombinedImageHash, ImageHashResult

class ImageHasher:
    size: int
    def __init__(self, size: int = 8):
        self.size = size

    def create_hash_from_image(self, image_path: Path) -> ImageHashResult:
        print(f"[HASHER] Opening {image_path}")
        ImageFile.LOAD_TRUNCATED_IMAGES = False

        try:
            phash, width, height = self.global_phash(image_path)

            print(f"[HASHER] Done {image_path}")
            return CombinedImageHash(
                path=image_path,
                hash=phash,
                width=width,
                height=height,
            ), None
        except (UnidentifiedImageError, OSError) as e:
            print(f"[HASHER] Error! {e}")
            return None, str(e)

    def alpharemover(self, image: Image.Image):
        if image.mode != 'RGBA':
            return image
        canvas = Image.new('RGBA', image.size, (255, 255, 255, 255))
        canvas.paste(image, mask=image)
        return canvas.convert('RGB')

    def _pil_grayscale_convert_to_np_arr(self, p: Path) -> tuple[np.ndarray, int, int]:
        with Image.open(p) as img:
            grayscale_img = img.convert('L')

            resized_img = grayscale_img.resize((self.size, self.size), Image.Resampling.NEAREST)
            del grayscale_img

            arr = np.array(resized_img, dtype=np.uint8)
            del resized_img

            return (arr, img.width, img.height)

    def global_phash(self, p: Path) -> tuple[imagehash.ImageHash, int, int]:
        """
        Converts an image into a perceptual hash.
        """
        print(f"[PHASH] loading array")
        arr, width, height = self._pil_grayscale_convert_to_np_arr(p)
        print(f"[PHASH] array loaded {arr.shape}")
        
        img = Image.new(mode="L", size=(self.size, self.size))
        quantiles = np.arange(100)
        print(f"[PHASH] computing percentile")
        quantiles_values = np.percentile(arr, quantiles)
        print(f"[PHASH] computing interp")
        zdata = (np.interp(arr, quantiles_values, quantiles) / 100 * 255).astype(np.uint8)
        print(f"[PHASH] done")
        img.putdata(zdata.flatten())

        hashed = imagehash.phash(image=img)
        del img

        return (hashed, width, height)

def imagehash_to_int(h: imagehash.ImageHash) -> int:
    arr = h.hash # pyright: ignore[reportUnknownMemberType, reportUnknownVariableType]
    assert isinstance(arr, np.ndarray)

    bits: np.typing.NDArray[np.bool_] = arr.astype(np.bool_, copy=False).ravel()

    packed: np.typing.NDArray[np.uint8] = np.packbits(bits)
    return int.from_bytes(packed.tobytes(), byteorder="big")



