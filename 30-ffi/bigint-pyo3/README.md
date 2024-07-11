# PyO3
PyO3 是 Rust 社区里非常火的与 Python 绑定的框架，它不仅可以实现用 Rust 给 Python 写扩展，还能在 Rust 的二进制程序中直接执行 Python 代码。所以实际 PyO3 是一个双向绑定库。

这样 PyO3 项目的一个关键是，需要使用 maturin 这种构建工具。你可以下载代码后，按下面的脚本准备好虚拟环境，并在这个虚拟环境里安装好 maturin。
```
$ cd bigint-pyo3
$ python -m venv .env
$ source .env/bin/activate
$ pip install maturin

maturin develop -r
```