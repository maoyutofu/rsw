extern crate comrak;
extern crate regex;
extern crate yaml_rust;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use parse::{MdFile, MdHead};
use util::*;

use self::comrak::{ComrakOptions, markdown_to_html};
use self::regex::Regex;
use self::yaml_rust::YamlLoader;

pub fn render(site_name: &str, public: &str, md_file: MdFile) {
    let yaml_docs = YamlLoader::load_from_str(md_file.yaml_str.as_str()).unwrap();
    let html_str = markdown_to_html(md_file.md_str.as_str(), &ComrakOptions::default());

    let yaml_doc = &yaml_docs[0];

    if yaml_doc["template"].as_str() == None {
        println!("- No template file specified: {}", md_file.file_name);
        process::exit(1);
    }
    let template = yaml_doc["template"].as_str().unwrap();

    if yaml_doc["title"].as_str() == None {
        println!("- No title: {}", md_file.file_name);
        process::exit(1);
    }
    let title = yaml_doc["title"].as_str().unwrap();

    let author = yaml_doc["author"].as_str().unwrap_or(site_name);

    let keywords = yaml_doc["keywords"].as_str().unwrap_or(site_name);


    let description = yaml_doc["description"].as_str().unwrap_or(title);

    let md_head = MdHead {
        template: template,
        title: title,
        author: author,
        keywords: keywords,
        description: description,
    };

    // 渲染模板
    let html_content = render_template(site_name, public, md_head, html_str.as_str(), md_file.page_id.as_str());

    // 生成目标文件
    generate_html(md_file.target_file_name.as_str(), html_content.as_str());
}

fn generate_html(html_path: &str, html_content: &str) {
    // 拆分文件名，如`build/2017/01/01/happy.html`得到的是`["happy.html", "build/2017/01/01"]`
    let dirs: Vec<&str> = html_path.rsplitn(2, '/').collect();
    create_not_exists(dirs[1]);

    let path = Path::new(html_path);
    let display = path.display();

    // 以只写模式打开文件，返回`io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(html_content.as_bytes()) {
        Err(why) => panic!("write {}: {}", display, why.description()),
        Ok(_) => println!("write {}", display),
    };
}

fn render_template(site_name: &str, public: &str, md_head: MdHead, html_str: &str, page_id: &str) -> String {
    // 从yaml数据中取出md文件的元数据
    let template = md_head.template;
    let template_names: Vec<&str> = template.rsplitn(2, '/').collect();
    let mut file_name = String::new();
    if template_names.len() == 1 {
        file_name.push_str("__");
        file_name.push_str(template);
    } else {
        file_name.push_str(template_names[1]);
        file_name.push_str("/__");
        file_name.push_str(template_names[0]);
    }

    let template_file = format!("{}/{}.html", public, file_name);

    // 打开模板文件
    let path = Path::new(template_file.as_str());
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file
    };

    let mut template_content = String::new();
    if let Err(err) = file.read_to_string(&mut template_content) {
        panic!("couldn't read {}: {}", display, err.description());
    }

    // 将site_name渲染到模板中
    let re_author = Regex::new(r"\{\{\s*site_name\s*\}\}").unwrap();
    template_content = String::from(re_author.replace_all(template_content.as_str(), site_name));

    // 将author渲染到模板中
    let re_author = Regex::new(r"\{\{\s*author\s*\}\}").unwrap();
    template_content = String::from(re_author.replace_all(template_content.as_str(), md_head.author));

    // 将title渲染到模板中
    let re_title = Regex::new(r"\{\{\s*title\s*\}\}").unwrap();
    template_content = String::from(re_title.replace_all(template_content.as_str(), md_head.title));

    // 将keywords渲染到模板中
    let re_keywords = Regex::new(r"\{\{\s*keywords\s*\}\}").unwrap();
    template_content = String::from(re_keywords.replace_all(template_content.as_str(), md_head.keywords));

    // 将description渲染到模板中
    let re_description = Regex::new(r"\{\{\s*description\s*\}\}").unwrap();
    template_content = String::from(re_description.replace_all(template_content.as_str(), md_head.description));

    // 将page_id渲染到模板中
    let re_content = Regex::new(r"\{\{\s*page_id\s*\}\}").unwrap();
    template_content = String::from(re_content.replace_all(template_content.as_str(), page_id));

    // 将content渲染到模板中
    let re_content = Regex::new(r"\{\{\s*content\s*\}\}").unwrap();
    template_content = String::from(re_content.replace_all(template_content.as_str(), html_str));

    return template_content;
}