# 标题还没想好

## 环境搭建

### 开发

1. 指定 rust 版本：1.54.0

```plain
rustup toolchain add 1.54.0 --profile minimal
rustup override set 1.54.0
```

然后快乐写 rust。注意不要用太奇怪的库就行了

测试程序的命令：

```plain
LD_LIBRARY_PATH=$PWD/project_hw/lib cargo test --lib -- --nocapture
```

好像指定版本后 cargo 从 crates.io 拉东西会卡住，不知道为什么

### 打包

1. 安装 mrustc

```plain
git clone https://github.com/thepowersgang/mrustc
cd mrustc
RUSTC_VERSION=1.54.0 make -f minicargo.mk
```

然后你会得到 `rustc-1.54.0-src` 这个文件夹，是标准库，还有 `bin/mrustc` 这个二进制，是等下要用到的妙妙工具

2. 使用 mrustc 编译项目

```plain
 LIBRARY_PATH=project_hw/lib MRUSTC_TARGET_VER=1.54 mrustc -L project_hw/rust --out-dir output src/main.rs
```

接着就能在 `output/main.c` 看到代码了。只要有文件就算成功，报错没关系

3. 集成运行

然后把生成的代码放到正确的位置……

```plain
mv output/main.c project_hw/algorithm/solve.c
```

后面像正常编译 cmake 项目一样对待 `hw_project` 就好了
