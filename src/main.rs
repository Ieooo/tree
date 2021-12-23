use tree::{fileSearch,treePrinter};
use std::path::PathBuf;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let path: String;
    if args.len() < 2 {
       path = String::from(".");
    } else {
        path = args[1].clone();
    }
    let fcontainer = fileSearch::FileContainer::new(PathBuf::from(path));
    for item in fcontainer {
        treePrinter::print_tree(item);
    }
}
