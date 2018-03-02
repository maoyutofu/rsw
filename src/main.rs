extern crate rsw;
extern crate regex;
#[macro_use]
extern crate clap;

use rsw::util::*;
use rsw::parse;
use rsw::template;

use regex::Regex;
use clap::App;

use std::fs;
use std::path::Path;
use std::error::Error;
use std::process;

fn copy_files(re_ignore: &Regex, target: &str, src: &str) {
    let dir = Path::new(src);
    // 遍历目录
    let entrys = match fs::read_dir(dir) {
        Err(why) => {
            println!("{}: {}", why.description(), src);
            process::exit(0x0100);
        },
        Ok(entrys) => entrys,
    };
    for entry in entrys {
        if let Ok(entry) = entry {
            let child = entry.path();
            let file_name = convert_path(child.to_str().unwrap());

            if child.is_file() {
                // 忽略匹配到的文件
                if re_ignore.is_match(file_name.as_str()) {
                    continue;
                }
                // 拆分源文件名，方便后面组合成目标文件名
                let dirs: Vec<&str> = file_name.splitn(2, '/').collect();
                let new_file = format!("{}/{}", target, dirs[1]);
                // 将目标文件从右边按`/`拆分得到目录
                let dirs:Vec<&str> = new_file.rsplitn(2, '/').collect();
                // 如果要复制的目标目录不存在，就创建
                create_not_exists(dirs[1]);
                // 复制文件
                match fs::copy(file_name.clone(), &new_file) {
                    Err(why) => panic!("{} -> {}: {}", file_name, new_file, why.description()),
                    Ok(_) => println!("{} -> {}", file_name, new_file),
                }
            } else {
                // 如果是目录，则继续递归
                copy_files(re_ignore, target, &file_name); 
            }
        }
    }
}

fn loop_parse(build: &str, public: &str, src: &str) {
    let path = Path::new(src);
    // 递归方式列出所有的源文件
    for entry in fs::read_dir(path).expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let child = entry.path();
            let file_name = child.to_str().unwrap();
            if child.is_file() {
                let re_md_file = Regex::new(r".*\.md$").unwrap();
                if !re_md_file.is_match(file_name) {
                    continue;
                }
                let md_file = parse::parse_md_file(build, &child);
                template::render(public, md_file);
            } else {
                loop_parse(build, public, file_name);
            }
        }
    }
}

// 编译后的模板文件目录，将可以发布到网上
static BUILD_DIR: &str = "build";
// 资源文件，如css、js、图片
static PUBLIC_DIR: &str = "public";
// 源文件目录
static SRC_DIR: &str = "src";

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let project_name = matches.value_of("PROJECT").unwrap();
        // 创建项目并初始化工作空间
        init_work_space(project_name, PUBLIC_DIR, SRC_DIR);
    }

    if let Some(_) = matches.subcommand_matches("build") {
        // copy public下的资源文件到build目录，但会忽略模板文件
        let re_template_file = Regex::new(r".*__.*\.html$").unwrap();
        copy_files(&re_template_file, BUILD_DIR, PUBLIC_DIR);
        let re_md_file = Regex::new(r".*\.md$").unwrap();
        // copy src下的资源文件到build目录，但会忽略.md文件
        copy_files(&re_md_file, BUILD_DIR, SRC_DIR);
        // 解析md文件
        loop_parse(BUILD_DIR, PUBLIC_DIR, SRC_DIR);
    }
}