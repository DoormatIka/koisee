
from pathlib import Path
import subprocess
import platform
import shutil

os_name = platform.system()
ext = ".exe" if os_name == "Windows" else ""

subprc_res = subprocess.run(["rustc", "--print", "host-tuple"], capture_output=True, text=True, check=True)
target = subprc_res.stdout.strip()
if len(target) <= 0:
    print("Failed to determine platform target triple.")

print(f"======[ BUILDING ON {target} ]=======")

print(f"======[ Building classifer. ]=======")
classifier_build_path = Path("classifier/").absolute()
classifier_name = f"classifier-{target}{ext}"
classifier_build_cmd = f"uv run nuitka --standalone --plugin-enable=numpy --plugin-enable=anti-bloat --include-package=imagehash --include-module=main --output-filename={classifier_name} main.py"

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
delete_all_files(target_dir)
print(f"Copying {source_dir} to {target_dir}.")
copy_dir_contents_to_another(source_dir, target_dir)

