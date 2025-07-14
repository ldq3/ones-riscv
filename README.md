
# OneS-RISCV
基于 [OneS](https://github.com/ldq3/ones) 实现的 RISC-V 平台上的操作系统。

## 快速开始
以 QEMU Virt 平台为例，编译和运行操作系统镜像。

安装依赖：
- Rust
- Tmux
- Mise
- qemu-system-riscv64 (≥ 7.2.0)

在 `kernel/.mise/task/start` 中设置变量 `FS_IMG` 的值为文件系统镜像的相对于项目根目录的路径。

运行任务：

```shell
cd kernel

LOG={level} mise run start
```

其中 level 可以是（大小写敏感）：
- error
- warn
- info
- debug
- trace

## 支持平台
- QEMU Virt