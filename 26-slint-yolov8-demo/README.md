# Slint Chatbot Demo

This is a demo of Rust + Slint + Candle + Yolov8, it looks like this:

![](./assets/objects.png)
![](./assets/poses.png)

## Do it by yourself

Make sure you have downloaded `yolov8m.safetensors` and `yolov8m-pose.safetensors` by:

```
HF_HUB_ENABLE_HF_TRANSFER=1 HF_ENDPOINT=https://hf-mirror.com huggingface-cli download lmz/candle-yolo-v8 yolov8m.safetensors
HF_HUB_ENABLE_HF_TRANSFER=1 HF_ENDPOINT=https://hf-mirror.com huggingface-cli download lmz/candle-yolo-v8 yolov8m-pose.safetensors
```
The downloads locate at `~/.cache/huggingface/hub/`.

Copy them to the root of the current project directory, like follows:

```
$ ls -lh
total 101M
-rwxr-xr-x 1 mike mike 150K Dec  9 14:58 Cargo.lock
-rwxr-xr-x 1 mike mike  615 Dec  9 14:58 Cargo.toml
-rwxr-xr-x 1 mike mike 1.1K Dec  9 14:58 LICENSE
-rwxr-xr-x 1 mike mike  747 Dec  9 18:29 README.md
drwxr-xr-x 2 mike mike 4.0K Dec  9 18:27 assets
-rwxr-xr-x 1 mike mike   71 Dec  9 14:58 build.rs
-rwxr-xr-x 1 mike mike 168K Dec  9 14:58 football.jpg
drwxr-xr-x 3 mike mike 4.0K Dec  9 14:58 src
drwxr-xr-x 4 mike mike 4.0K Dec  9 15:03 target
drwxr-xr-x 2 mike mike 4.0K Dec  9 14:58 ui
-rw-r--r-- 1 mike mike  51M Dec  9 17:45 yolov8m-pose.safetensors
-rw-r--r-- 1 mike mike  50M Dec  9 17:45 yolov8m.safetensors
```

and then 

```
cargo run --release
```

You will look at a GUI app popped up, good luck!


# 思考题

这节课的代码实现有一个性能上的问题，就是每次点击 Detect Objects 或 Detect Poses 的时候，实际上都重复加载了模型，你想一想如何优化这个点？欢迎你把你的想法和优化代码分享到评论区，如果你觉得对你有帮助的话，也欢迎你把这节课的内容分享给其他朋友，我们下节课再见！