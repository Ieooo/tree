use crate::fileItem::FileItem;
use std::path::PathBuf;
use std::vec::Vec;

pub struct FileContainer {
    pub stack: Vec<FileItem>
}

impl FileContainer {
    pub fn new(file_path: PathBuf) -> Self {
        let mut stack: Vec<FileItem> = Vec::new();
        stack.push(FileItem::new(file_path));
        FileContainer {
            stack
        }
    }
}

impl Iterator for FileContainer {
    type Item = FileItem;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.stack.is_empty() {
            let top = self.stack.pop().unwrap();
            if top.file_path.is_dir() {
                for (i,item) in top.file_path.read_dir().unwrap().enumerate() {
                    let mut file_item = FileItem::new(item.unwrap().path());
                    file_item.level = top.level + 1;
                    file_item.not_last = top.not_last.clone();
                    if i != 0 {
                        file_item.not_last.push(file_item.level); 
                    }
                    self.stack.push(file_item);
                }
            }
            return Some(top);
        } else {
            None
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_iterator() {
        let container = FileContainer::new(PathBuf::from("./"));
        for i in container {
            println!("{:?}", i.file_path);
        } 
    }
}