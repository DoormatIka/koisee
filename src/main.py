import os
from pathlib import Path

import hasher
import logger
import argparse

parser = argparse.ArgumentParser(description="Fuzzy duplicate image finder.")
_ = parser.add_argument("-i", "--input", help="A folder to scan.")
_ = parser.add_argument("-d", "--delete", action='store_true', help="Enable automatic deletion of files.")

def scan_from_directory(directory: Path, is_delete: bool = False): # prototype
    print(f"[START] - Parsing through {directory}")

    imghasher = hasher.ImageHasher(logger=logger.MatchLogger(), size=16)
    finder = hasher.BruteForceFinder(hasher=imghasher)

    hashes = finder.create_hashes_from_directory(directory)
    similar_images = finder.get_similar_images(hashes)
    if is_delete:
        for img1, img2 in similar_images:
            # extremely basic quality check, replace with something else later.
            if img1.pixel_count >= img2.pixel_count:
                os.remove(img2.path)
            else:
                os.remove(img1.path)


def main():
    args = parser.parse_args()
    is_delete = bool(args.delete) # pyright: ignore[reportAny]
    inp = str(args.input) # pyright: ignore[reportAny]
    if inp:
        dir_path = Path(inp)
        if dir_path.is_dir():
            _ = scan_from_directory(dir_path, is_delete=is_delete)
            print("Finished.")
        else:
            print("[ERROR] - Please pass in a directory.")
    else:
        parser.print_help()


if __name__ == "__main__":
    main()

