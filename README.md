# RustWriter
这是一个用Rust语言编写的静态博客生成工具。追求简单、自由、快乐。

### 安装
1. 源码方式安装
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
2.  二进制方式安装
    - 从 [release](https://github.com/tjz101/rsw/releases) 页面中下载对应平台的可执行文件压缩包解压后就可以使用。
	
    - 为方便使用，可以将rsw添加到环境变量中 或者 Linux系统将rsw文件放到/usr/local/bin，Windows系统将rsw.exe放到C:\Windows\System32中。

### 使用

- rsw -h 查看帮助
- rsw -V 显示版本信息
- rsw new project 创建一个静态博客项目
- rsw build 编译src目录下的文件到build

### 部署
将build目录下的文件上传到你的服务器就可以了。

### 案例
- [myblog](https://jizhong.plus/)
- [rsw-example](http://dev-tang.gitee.io/pages/rsw-example/) - [src](https://github.com/tjz101/rsw-example)
- [rust-case-study-manual](https://www.irust.org/rust-case-study-manual/)

### 感悟
最近开始学习Rust，难不难学不重要，关键是知道自己想干什么。尽管写完这个项目，我脑袋里充满了unwrap()，但还是很开心，让我迈出了坚实的一步。

### 特别说明
程序中的错误提示都是用Google翻译直接翻译的，不通的地方请见谅。
