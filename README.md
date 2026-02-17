<h1 align="center">koisee</h1>
<p align="center">Let Koishi do her best finding images for you 🌸</p>
<p align="center">
    <img src="./koishilogo.jpg" />
</p>

## what is this?

**koisee** is a small, experimental tool that helps you find duplicate or similar images in folders.

she's still learning, so she isn't available yet... please wait warmly while she's preparing

## future...

koisee wants to:

- support gifs, videos and audio
- becoming faster and less memory intensive
- learn how to use profilers and flamegraphs for the above...
- have CI/CD and automated testing..

though images are the main focus!

## building

todo: make a `just` recipe for all of this.

```
cd koisee
cargo tauri build --release

cd classifier
uv sync
uv run nuitka --standalone --plugin-enable=numpy --plugin-enable=anti-bloat --include-package=imagehash --include-module=main --python-flag=no_docstrings main.py
```
