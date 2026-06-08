<h1 align="center">koisee</h1>
<p align="center">Let Koishi do her best finding images for you 🌸</p>
<p align="center">
    <img src="./koishilogo.jpg" />
</p>

## what is this?

**koisee** is a small, fairly fast, and experimental tool that helps you find duplicate or similar images in folders.

she's still learning, so she isn't available yet... please wait warmly while she's preparing

## future...

koisee wants to:

- support gifs, videos and audio
- use neural networks for more accurate matching
- becoming faster and less memory intensive
- learn how to use profilers and flamegraphs for the above...
- have CI/CD and automated testing..

though images are the main focus!

## limitations.

matching gets slower as more images (5,000+) gets scanned since this is using a coarse clustering technique + brute force.
This isn't the final implementation, this app is experimental.

## building

do `npm run tauri build`
