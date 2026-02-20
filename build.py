
import os
from pathlib import Path
import subprocess
import platform
import shutil
import argparse

parser = argparse.ArgumentParser()
_ = parser.add_argument("-sn", "--skip-nuitka", help="Skip nuitka compilation", action="store_true")
_ = parser.add_argument("-st", "--skip-tauri", help="Skip tauri compilation", action="store_true")
_ = parser.add_argument("-rt", "--run-tauri", help="Run tauri app in dev.", action="store_true")
args = parser.parse_args()
skip_nuitka: bool = args.skip_nuitka # pyright: ignore[reportAny]
skip_tauri: bool = args.skip_tauri # pyright: ignore[reportAny]
run_tauri: bool = args.run_tauri # pyright: ignore[reportAny]


os_name = platform.system()
ext = ".exe" if os_name == "Windows" else ""

subprc_res = subprocess.run(["rustc", "--print", "host-tuple"], capture_output=True, text=True, check=True)
target = subprc_res.stdout.strip()
if len(target) <= 0:
    print("Failed to determine platform target triple.")

print(f"======[ BUILDING ON {target} ]=======")


if not skip_nuitka:
    print(f"======[ Building classifer. ]=======")
    classifier_build_path = Path("classifier/").absolute()
    classifier_name = f"classifier-{target}{ext}"
    classifier_build_cmd = f"uv run nuitka --standalone --nofollow-import-to=tkinter --nofollow-import-to=_tkinter --noinclude-dll=\"libtcl*\" --noinclude-dll=\"libtk*\" --plugin-enable=numpy --plugin-enable=anti-bloat --include-package=imagehash --include-module=main --output-filename={classifier_name} main.py"
    _ = subprocess.run(["uv", "sync"], cwd=classifier_build_path)
    _ = subprocess.run(classifier_build_cmd.split(" "), cwd=classifier_build_path)


print(f"======[ Transferring python build to tauri. ]=======")
source_dir = Path("classifier/main.dist/")
target_dir = Path("koisee/src-tauri/binaries/classifier-bundle")

def delete_all_files(dir: Path):
    for item in dir.iterdir():
        try:
            if item.is_file() or item.is_symlink():
                item.unlink()
            elif item.is_dir():
                shutil.rmtree(item)
        except Exception as e:
            print(f"Failed to delete {item.name}: {e}")

def copy_dir_contents_to_another(source_dir: Path, dest_dir: Path):
    for f in source_dir.glob("*"):
        dest_path = dest_dir / f.name
        if f.is_dir():
            _ = shutil.copytree(f, dest_path, dirs_exist_ok=True)
        else:
            _ = shutil.copy(f, dest_dir / f.name)

print(f"Deleting destination folder \"{target_dir}\" contents.")
if not os.path.exists(target_dir):
    os.makedirs(target_dir)

delete_all_files(target_dir)
print(f"Copying {source_dir} to {target_dir}.")
copy_dir_contents_to_another(source_dir, target_dir)


if not skip_tauri:
    print(f"======[ building tauri application. ]=======")
    koisee_path = Path("koisee/").absolute()
    if os_name == "Windows":
        print(f"building portable windows binary.")
        _ = subprocess.run(["npm", "run", "tauri", "build", "--", "--no-bundle"], cwd=koisee_path)
    else:
        print(f"bundling deb file + portable version.")
        _ = subprocess.run(["npm", "run", "tauri", "build", "--", "--verbose"], cwd=koisee_path)

if run_tauri:
    print(f"======[ run tauri application. ]=======")
    koisee_path = Path("koisee/").absolute()
    _ = subprocess.run(["npm", "run", "tauri", "dev"], cwd=koisee_path)
