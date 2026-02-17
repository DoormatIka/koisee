
from pathlib import Path
import subprocess
import platform

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
classifier_build_cmd = f"uv run nuitka --standalone --plugin-enable=numpy --plugin-enable=anti-bloat --include-package=imagehash --include-module=main --python-flag=no_docstrings --output-filename={classifier_name} main.py"

_ = subprocess.run(["uv", "sync"], cwd=classifier_build_path)
_ = subprocess.run(classifier_build_cmd.split(" "), cwd=classifier_build_path)

print(f"======[ Transferring python build to tauri. ]=======")
"""
tauri.conf.json
https://v2.tauri.app/develop/resources/
https://v2.tauri.app/develop/sidecar/
{
  "bundle": {
    "externalBin": [
      "binaries/classifier" 
    ],
    "resources": [
      "binaries/*"
    ]
  }
}
"""
