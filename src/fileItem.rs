use std::path::PathBuf;

#[derive(Debug)]
pub struct FileItem {
    pub file_name: String,         // 文件/目录名
    pub file_path: PathBuf,        // 文件/目录路径
    pub level: i32,                // 文件相对于查询目录的层级深度
    pub not_last: Vec<i32>,        // 目录中不是最后一个的level集合
    pub is_last: bool,             // 文件/目录是否是同级中最后一个
}

impl FileItem {
    pub fn new(file_path: PathBuf) -> Self {
        FileItem {
            file_name: match file_path.file_name() {
                Some(name) => name.to_str().unwrap().to_string(),
                None => file_path.to_str().unwrap().to_string()
            },
            file_path,
            level: 0,
            not_last: Vec::new(),
            is_last: false,
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_fileitem() {
        let file_item = FileItem::new(PathBuf::from("hello/java/jdk"));
        print!("{:?}", file_item);
    }
}