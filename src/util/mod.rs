extern crate regex;

use self::regex::Regex;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

pub fn create_not_exists(dir: &str) {
    if !Path::new(dir).exists() {
        match fs::create_dir_all(dir) {
            Err(why) => panic!("create {}: {}", dir, why.description()),
            Ok(_) => println!("create {}", dir),
        };
    }
}

pub fn write_file(file_name: &str, content: &str) {
    let index_md = Path::new(&file_name);
    let mut file_index_md = match File::create(index_md) {
        Err(why) => panic!("create {}: {}", file_name, why.description()),
        Ok(file) => file,
    };
    match file_index_md.write_all(content.as_bytes()) {
        Err(why) => panic!("write {}: {}", file_name, why.description()),
        Ok(_) => println!("write {}", file_name),
    };
}

static MD_STR: &str = r#"---
title: {{ project }} 
author: RustWriter
template: index
---

# {{ project }}
This is written in rust writer. Simple, free and happy."#;

static HTML_STR: &str = r#"<!DOCTYPE html>
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

pub fn init_work_space(project_name: &str, public_dir: &str, src_dir: &str) {
    let path = Path::new(project_name);
        if path.exists() {
            println!("{} exists", project_name);
            process::exit(0x0100);
        }
        let project_src = format!("{}/{}", project_name, src_dir);
        let project_public = format!("{}/{}", project_name, public_dir);
        create_not_exists(&project_src);
        create_not_exists(&project_public);

        let index_md_name = format!("{}/{}", &project_src, "index.md");
        let re_project = Regex::new(r"\{\{\s*project\s*\}\}").unwrap();
        let md_text = String::from(re_project.replace_all(MD_STR, project_name));
        write_file(&index_md_name, &md_text);
        let index_tpl_name = format!("{}/{}", &project_public, "__index.html");
        
        write_file(&index_tpl_name, HTML_STR);
        println!("{} created successfully", project_name);
}