use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::utils::{lines, matches, read_input};

type WeakFolder = Weak<RefCell<Folder>>;
type RcFolder = Rc<RefCell<Folder>>;

#[derive(Debug)]
enum TerminalLine {
    CdTo(String),
    CdBack,
    Ls,
    Dir(String),
    File(String, usize),
    Nop,
}
impl From<&String> for TerminalLine {
    fn from(line: &String) -> Self {
        if matches(line, "\\$ cd (/|\\w+)") {
            TerminalLine::CdTo(line.replace("$ cd ", ""))
        } else if matches(line, "\\$ cd ..") {
            TerminalLine::CdBack
        } else if matches(line, "\\$ ls") {
            TerminalLine::Ls
        } else if matches(line, "dir \\w") {
            TerminalLine::Dir(line.replace("dir ", ""))
        } else if matches(line, "\\d+ \\w") {
            let data: Vec<&str> = line.split(" ").collect();
            let first = data[0].parse::<usize>().unwrap();
            let second = data[1];
            TerminalLine::File(second.to_owned(), first)
        } else {
            TerminalLine::Nop
        }
    }
}

#[derive(Debug)]
struct Folder {
    parent: WeakFolder,
    name: String,
    size: usize,
    folders: HashMap<String, RcFolder>,
}
impl Folder {
    fn new(name: String, parent: WeakFolder) -> Self {
        Self {
            name,
            parent,
            size: 0,
            folders: HashMap::new(),
        }
    }
    fn add(&mut self, folder: RcFolder) {
        self.folders
            .insert(folder.borrow().name.clone(), Rc::clone(&folder));
    }
    fn get(&mut self, folder_name: String) -> &RcFolder {
        self.folders.get(&folder_name).unwrap()
    }
    fn increase_size(&mut self, size: usize) {
        self.size += size;
        Weak::upgrade(&self.parent).map(|it| it.borrow_mut().increase_size(size));
    }
}

pub fn run() {
    let data = lines(read_input(7));
    let mut iter = data.iter().map(TerminalLine::from);

    iter.next();

    let folder = Rc::new(RefCell::new(Folder::new("/".to_owned(), Weak::new())));
    build_tree(&folder, &mut iter);
    println!("{:?}", part1(&folder, 100000));
}

fn part1(folder: &RcFolder, size: usize) -> usize {
    folder
        .borrow()
        .folders
        .values()
        .into_iter()
        .map(|it| {
            let it_size = it.borrow().size;
            if it_size < size {
                it_size + part1(it, size)
            } else {
                part1(it, size)
            }
        })
        .sum()
}

fn build_tree<'a>(folder: &RcFolder, iter: &'a mut impl Iterator<Item = TerminalLine>) {
    if let Some(line) = iter.next() {
        match line {
            TerminalLine::CdTo(subdir) => {
                let subfolder = Rc::clone(folder.borrow_mut().get(subdir));
                build_tree(&subfolder, iter);
            }
            TerminalLine::CdBack => {
                build_tree(&Weak::upgrade(&folder.borrow().parent).unwrap(), iter)
            }
            TerminalLine::Dir(name) => {
                let subdolder = Folder::new(name, Rc::downgrade(&folder));
                folder.borrow_mut().add(Rc::new(RefCell::new(subdolder)));

                build_tree(folder, iter);
            }
            TerminalLine::File(_name, size) => {
                folder.borrow_mut().increase_size(size);

                build_tree(folder, iter);
            }
            _ => {
                build_tree(folder, iter);
            }
        }
    }
}
