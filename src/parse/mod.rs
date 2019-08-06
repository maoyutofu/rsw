extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use util::*;

use self::regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
pub struct MdFile {
    pub file_name: String,
    pub target_file_name: String,
    pub page_id: String,
    pub yaml_str: String,
    pub md_str: String,
}

impl MdFile {
    pub fn new<T: Into<String>>(file_name: T, target_file_name: T, page_id: T, yaml_str: T, md_str: T) -> Self {
        MdFile {
            file_name: file_name.into(),
            target_file_name: target_file_name.into(),
            page_id: page_id.into(),
            yaml_str: yaml_str.into(),
            md_str: md_str.into(),
        }
    }
}

pub fn parse_md_file(build: &str, path: &Path) -> MdFile {
    let display = path.display();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error`的`description`方法返回一个描述错误的字符串
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回`io::Result<usize>`
    let mut content = String::new();
    if let Err(err) = file.read_to_string(&mut content) {
        panic!("couldn't read {}: {}", display, err);
    }
    // `file`离开作用域，文件将被关闭

    let re_md = Regex::new(r"^---([\s\S]*?)---([\s\S]*)").unwrap();
    let caps = re_md.captures(content.as_str()).unwrap();
    let yaml_str = caps.get(1).unwrap().as_str();
    // 提取正文markdown内容
    let mut md_str = String::from(caps.get(2).unwrap().as_str());
    // 提取rsw://格式转成超链接
    let re_rsw = Regex::new(r"(\(rsw://(?P<url_str>\S+)\.md(?P<query_str>[\S]*)\))").unwrap();
    md_str = String::from(re_rsw.replace_all(&md_str, "($url_str.html$query_str)"));

    // let file_name = path.to_str().unwrap();
    let file_name = convert_path(path.to_str().unwrap());
    // 将src路径转成build路径
    let file_names: Vec<&str> = file_name.splitn(2, '/').collect();
    let target_file = format!("{}/{}", build, file_names[1]);
    // 将md扩展转成html
    let target_files: Vec<&str> = target_file.rsplitn(2, '.').collect();
    let target_file_name = format!("{}{}", target_files[1], ".html");

    // 得到page_id
    let file_names: Vec<&str> = file_names[1].rsplitn(2, '.').collect();
    let page_id = file_names[1].replace("/", "_");
    return MdFile::new(file_name.clone(), target_file_name, page_id, String::from(yaml_str), md_str);
}

#[derive(Debug)]
#[derive(Clone)]
pub struct MdHead<'a> {
    pub template: &'a str,
    pub title: &'a str,
    pub author: &'a str,
    pub keywords: &'a str,
    pub description: &'a str,
    pub ctime: &'a str,
    pub mtime: &'a str
}