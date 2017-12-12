# RustWriter
这是一个用Rust语言编写的静态博客生成工具。追求简单、自由、快乐。

### 安装
- 下载源代码
```
git clone https://github.com/tjz101/rsw.git
cd rsw
```
- 编译代码
```
cargo build --release
```
- Linux 安装
```
sudo cp target/release/rsw /usr/local/bin/
```

### 使用

- rsw -h 查看帮助
- rsw -V 显示版本信息
- rsw new project 创建一个静态博客项目
- rsw build 编译src目录下的文件到build

### 举个栗子
[rsw-example](http://dev-tang.gitee.io/pages/rsw-example/) - [src](https://github.com/tjz101/rsw-example)

### 感悟
最近开始学习Rust，难不难学不重要，关键是知道自己想干什么。尽管写完这个项目，我脑袋里充满了unwrap()，但还是很开心，让我迈出了坚实的一步。
