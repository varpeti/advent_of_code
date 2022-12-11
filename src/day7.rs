use std::{collections::HashMap, rc::Rc, cell::RefCell};

use regex::Regex;

type Files = HashMap<String, u32>;
type Dirs = HashMap<String, Rc<RefCell<Dir>>>;

#[derive(Debug)]
struct Dir {
    files: Files,
    dirs: Dirs,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Dir{files: Files::new(), dirs: Dirs::new(), parent: None}))
    }
    fn add_file(&mut self, file_name: String, file_size: u32) {
        self.files.insert(file_name, file_size);
    }
    fn add_dir(parent_dir: Rc<RefCell<Dir>>, child_name: String) {
        let child = Dir::new();
        parent_dir.borrow_mut().dirs.insert(child_name, child.clone());
        child.borrow_mut().parent = Some(parent_dir);
    }
    fn get_dir(&self, dir_name: String) -> Option<Rc<RefCell<Dir>>> {
        match dir_name.as_str() {
            ".." => self.parent.clone(),
            _ => self.dirs.get(&dir_name).map(|dir| dir.clone()),
        }
    }

    /* fn get_size(&self) -> u32 {
        self.files.iter().map(|(_name, size)| size).sum::<u32>()
            +
            self.dirs.iter().map(|(_name, dir)| dir.borrow().get_size()).sum::<u32>()
    } */

    fn get_sizes(&self, path: String, sizes: &mut HashMap<String, u32>) -> u32 {
        let size = self.files.iter().map(|(_name, size)| size).sum::<u32>() +
                   self.dirs.iter().map(|(name, dir)| 
                    dir.borrow().get_sizes(format!("{}/{}",path, name.clone()), sizes)).sum::<u32>();
        sizes.insert(path, size);
        size
    }

}

const AT_MOST_SIZE: u32 = 100000;
const MAX_SPACE: u32 = 70000000;
const NEED_SPACE: u32 = 30000000;

fn parse_input(input: &String) -> Rc<RefCell<Dir>> {
    let file_re = Regex::new(r"^(\d*) (.*)$").expect("bad regex file");
    let dir_re = Regex::new(r"^dir (.*)$").expect("bad regex dir");
    let command_cd_re = Regex::new(r"^\$ cd (.*)$").expect("bad regex cd");
    // let command_ls_re = Regex::new(r"^\$ ls$").expect("bad regex ls");
    
    let root = Dir::new();
    let mut current_dir = root.clone();
    for line in input.lines().skip(1) {
        /*dbg!(line);*/

        if let Some(file) = file_re.captures(line) {
            let file_name = file.get(2).expect("file name").as_str().to_string();
            let file_size = file.get(1).expect("file size").as_str().parse::<u32>().expect("file size parse");
            /*dbg!(&file_name, &file_size);*/
            current_dir.borrow_mut().add_file(file_name, file_size);
            continue;
        }

        if let Some(dir) = dir_re.captures(line) {
            let dir_name = dir.get(1).expect("dir name").as_str().to_string();
            /*dbg!(&dir_name);*/
            Dir::add_dir(current_dir.clone(), dir_name);
            continue;
        }

        if let Some(cd) = command_cd_re.captures(line) {
            let dir_name = String::from(cd.get(1).expect("dir name").as_str());
            /*dbg!(&dir_name);*/
            current_dir = current_dir.clone().borrow().get_dir(dir_name).expect("dir not found");
            continue;
        }

        //if let Some(ls) = command_ls_re.captures(line) {}
    }
    root
}

pub fn day7_a(input: &String) -> String {
    let root = parse_input(input);
    let mut sizes = HashMap::new();
    root.borrow().get_sizes(String::from(""), &mut sizes);
    /*dbg!(&sizes);*/
    format!("{:?}", sizes.iter().map(|(_name, &size)| size).filter(|&size| size <= AT_MOST_SIZE).sum::<u32>())
}

pub fn day7_b(input: &String) -> String {
    let root = parse_input(input);
    let mut sizes = HashMap::new();
    root.borrow().get_sizes(String::from(""), &mut sizes); 
    /*dbg!(&sizes);*/
    let min_delete_size = NEED_SPACE - (MAX_SPACE - sizes.get("").expect("root missing?"));
    /*dbg!(min_delete_size);*/
    format!("{}", sizes.iter().map(|(_name, &size)| size).filter(|&size| size >= min_delete_size).min().expect("min"))
}
