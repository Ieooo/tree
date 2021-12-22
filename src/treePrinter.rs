use crate::fileItem::FileItem;

// 绘制目录结构
pub fn print_tree(file_item: FileItem) {
    let mut line_str = String::new();
    for j in 0..file_item.level {
        if file_item.not_last.contains(&(j+1)) {
            line_str.push_str("|   ");
        } else if j < file_item.level - 1{
            line_str.push_str("    ");
        }
    }
    if file_item.level != 0 {
        if file_item.is_last {
            line_str.push_str("`--");
        } else {
            line_str.push_str("|--");
        }
    } 
    line_str.push_str(format!("{}", file_item.file_name).as_str());
    println!("{}", line_str);
}
