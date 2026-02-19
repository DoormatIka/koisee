<h1 align="center">koisee</h1>
<p align="center">Let Koishi do her best finding images for you 🌸</p>
<p align="center">
    <img src="./koishilogo.jpg" />
</p>

## what is this?

**koisee** is a small, experimental tool that helps you find duplicate or similar images in folders.

she's still learning, so she isn't available yet... please wait warmly while she's preparing

this is also my first time making a multi-language application.

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
python build.py
cd koisee
env APPIMAGE_EXTRACT_AND_RUN=1 npm run tauri build -- --verbose
```
