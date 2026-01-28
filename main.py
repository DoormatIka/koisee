from dataclasses import dataclass
import numpy as np
import annoy as ann
import tkinter as tk
import imagehash

from PIL import Image, ImageTk
from PIL.Image import Image as PILImage
from pathlib import Path


tkwin = tk.Tk()

def view_image(image: PILImage):
    new_win = tk.Toplevel(tkwin)
    new_win.title("img display")
    new_win.geometry("400x400")

    image_label = tk.Label(new_win)
    image_label.pack()
    display_image = image.resize((400, 400), Image.Resampling.NEAREST)
    tk_img = ImageTk.PhotoImage(display_image)
    _ = image_label.config(image=tk_img)
    setattr(image_label, "image", tk_img)


# issues:
# combine phash (global) and crop_hash (per-section) 
# need to check what the crop_resistant_hash actually crops
#   what i mean by this is: what the images it passes to the hash function inside crop_resistant_hash
#   so i can tune it.
# please don't use chatgpt pleaase please.

def alpharemover(image: Image.Image):
	if image.mode != 'RGBA':
		return image
	canvas = Image.new('RGBA', image.size, (255, 255, 255, 255))
	canvas.paste(image, mask=image)
	return canvas.convert('RGB')

def global_phash(size: int):
    def function(img: PILImage):
        img = img.convert('L').resize((size, size), Image.Resampling.NEAREST)

        data = np.ascontiguousarray(img.get_flattened_data()).reshape(-1)
        quantiles = np.arange(100)
        quantiles_values = np.percentile(data, quantiles)
        zdata = (np.interp(data, quantiles_values, quantiles) / 100 * 255).astype(np.uint8)
        img.putdata(zdata)

        view_image(img)

        return imagehash.phash(image=img)

    return function

def crop_resistant_hash(img: PILImage) -> imagehash.ImageMultiHash:
    image = alpharemover(img)
    return imagehash.crop_resistant_hash(image=image, min_segment_size=100) # pyright: ignore[reportUnknownMemberType]

def print_matches(nearest_matches: list[list[ImagePath]]):
    for matches in nearest_matches:
        img1 = matches[0]
        img2 = matches[1]
        print(f"Left: {img1.path}\nRight: {img2.path}\nDifference: {abs(img1.hash - img2.hash)}\n\n")

def imagehash_to_int(h: imagehash.ImageHash) -> int:
    arr = h.hash
    assert isinstance(arr, np.ndarray)

    bits: np.typing.NDArray[np.bool_] = arr.astype(np.bool_, copy=False).ravel()

    packed: np.typing.NDArray[np.uint8] = np.packbits(bits)
    return int.from_bytes(packed.tobytes(), byteorder="big")

@dataclass
class ImagePath:
    path: Path
    hash: imagehash.ImageHash

def mainloop(directory: str): # prototype
    exts = {"png", "jpg", "jpeg"}
    make_phash = global_phash(16)
    image_hashes: list[ImagePath] = list()

    for ext in exts:
        image_paths = Path(directory).rglob(f"*.{ext}")
        for image_path in image_paths:
            img = Image.open(image_path)
            phash = make_phash(img)
            image_hashes.append(ImagePath(image_path, phash))

            print(f"path: {image_path}")

    image_hashes.sort(key=lambda x: imagehash_to_int(x.hash))

    nearest_matches: list[list[ImagePath]] = list()
    MATCH_THRESHOLD = 5
    for i, img1 in enumerate(image_hashes):
        for img2 in image_hashes[i + 1:]:
            if img1.path == img2.path:
                continue
            if abs(img1.hash - img2.hash) < MATCH_THRESHOLD:
                nearest_matches.append([img1, img2])

    print("\n\nNearest Matches: ")
    print_matches(nearest_matches)


def main():
    mainloop("/home/mualice/Downloads/")
    # tkwin.mainloop()


if __name__ == "__main__":
    main()

