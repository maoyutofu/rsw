extern crate rsw;
extern crate regex;
#[macro_use]
extern crate clap;

use rsw::util::*;
use rsw::parse;
use rsw::template;

use regex::Regex;
use clap::App;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

fn copy_public(target: &str, src: &str) {
    let dir = Path::new(src);
    // 遍历目录
    for entry in fs::read_dir(dir).expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let child = entry.path();
            let file_name = child.to_str().unwrap();

            if child.is_file() {
                // 判断如果是模板文件就忽略
                let re_template_file = Regex::new(r".*__.*\.html$").unwrap();
                if re_template_file.is_match(file_name) {
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
                match fs::copy(file_name, &new_file) {
                    Err(why) => panic!("{} -> {}: {}", file_name, new_file, why.description()),
                    Ok(_) => println!("{} -> {}", file_name, new_file),
                }
            } else {
                // 如果是目录，则继续递归
                copy_public(target, file_name); 
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
        let path = Path::new(project_name);
        if path.exists() {
            println!("{} exists", project_name);
            process::exit(0x0100);
        }
        let project_src = format!("{}/{}", project_name, SRC_DIR);
        let project_public = format!("{}/{}", project_name, PUBLIC_DIR);
        create_not_exists(&project_src);
        create_not_exists(&project_public);
        let index_md_name = format!("{}/{}", &project_src, "index.md");
        let md = r#"---
title: MyBlog 
author: RustWriter
template: index
---

# MyBlog
This is written in rust writer. Simple, free and happy."#;
        write_file(&index_md_name, md);
        let index_tpl_name = format!("{}/{}", &project_public, "__index.html");
        let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <meta name="author" content="{{ author }}" />
    <title>{{ title }}</title>
</head>
<body>
    {{ content }}
</body>
</html>"#;
        write_file(&index_tpl_name, html);
        println!("{} created successfully", project_name);
    }

    if let Some(_) = matches.subcommand_matches("build") {
        // copy public下的资源文件到build目录，但会忽略模板文件
        copy_public(BUILD_DIR, PUBLIC_DIR);
        // 解析md文件
        loop_parse(BUILD_DIR, PUBLIC_DIR, SRC_DIR);
    }
}