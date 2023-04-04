use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

const INPUT: &str = include_str!("input.txt");

// const INPUT: &str = "$ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k";

lazy_static! {
    static ref FILE_REX: Regex = Regex::new(r"(?P<size>\d+) (?P<name>.+)").unwrap();
    static ref DIR_REX: Regex = Regex::new(r"dir (?P<name>.+)").unwrap();
    static ref CD_REX: Regex = Regex::new(r"\$ cd (?P<name>.+)").unwrap();
}

#[derive(Debug, Default, Eq, PartialEq, Hash)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Default)]
struct RCFolder(RefCell<Folder>);

impl Hash for RCFolder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state)
    }
}

impl PartialEq<Self> for RCFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for RCFolder {}

#[derive(Debug, Default, Eq, PartialEq, Hash)]
struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<Rc<RCFolder>>,
}

fn parse(input: &str) -> Folder {
    let folder = Rc::new(RCFolder(RefCell::new(Folder {
        name: "/".to_string(),
        files: vec![],
        folders: vec![],
    })));
    let mut lines = input.lines();
    let mut folder_stack = Vec::new();
    folder_stack.push(folder.clone());
    if lines.next().expect("a line") != "$ cd /" {
        panic!("unsupported start")
    }
    for line in lines {
        // dbg!(line);
        // dbg!(&folder_stack);
        // dbg!(&folder);
        if let Some(cap) = FILE_REX.captures(line) {
            folder_stack
                .last_mut()
                .expect("at least one folder")
                .0
                .borrow_mut()
                .files
                .push(File {
                    name: cap.name("name").expect("a name").as_str().to_string(),
                    size: cap
                        .name("size")
                        .expect("a size")
                        .as_str()
                        .parse()
                        .expect("a num"),
                });
        } else if let Some(cap) = DIR_REX.captures(line) {
            folder_stack
                .last_mut()
                .expect("at least one folder")
                .0
                .borrow_mut()
                .folders
                .push(Rc::new(RCFolder(RefCell::new(Folder {
                    name: cap.name("name").expect("a name").as_str().to_string(),
                    files: vec![],
                    folders: vec![],
                }))));
        } else if let Some(cap) = CD_REX.captures(line) {
            // println!(".");
            let name = cap.name("name").expect("a name").as_str().to_string();
            if name == ".." {
                folder_stack.pop();
            } else {
                let the_folder = folder_stack
                    .last()
                    .expect("at least one folder")
                    .clone()
                    .0
                    .borrow()
                    .folders
                    .iter()
                    .find(|f| f.0.borrow().name == name)
                    .expect("folder exists")
                    .clone();
                folder_stack.push(the_folder);
                // dbg!(&folder_stack);
            }
        }
    }
    folder.0.take()
}

impl Folder {
    fn size(&self) -> usize {
        self.folders.iter().fold(0, |s, f| s + f.0.borrow().size())
            + self.files.iter().fold(0, |s, f| s + f.size)
    }
}

fn dir_sizes(root: Rc<RCFolder>) -> HashMap<Rc<RCFolder>, usize> {
    let mut ret = HashMap::new();
    ret.insert(root.clone(), root.0.borrow().size());
    for f in root.0.borrow().folders.iter() {
        let f = f.clone();
        ret.extend(dir_sizes(f));
    }
    ret
}

#[test]
fn part1() {
    let result = parse(INPUT);
    // println!("{:?}", result);
    // println!("{:?}", result.size());
    let result = dir_sizes(Rc::new(RCFolder(RefCell::new(result))));
    println!(
        "{:?}",
        result
            .iter()
            .filter_map(|(_, &s)| if s <= 100000 { Some(s) } else { None })
            .sum::<usize>()
    );
}

#[test]
fn part2() {
    let result = parse(INPUT);
    let system_size = result.size();
    dbg!(system_size);
    let avail_size = 70000000 - system_size;
    dbg!(avail_size);
    let needed_size = 30000000 - avail_size;
    dbg!(needed_size);
    let result = dir_sizes(Rc::new(RCFolder(RefCell::new(result))));
    dbg!(&result);
    let mut result = result
        .iter()
        .filter_map(|(_, &s)| if s >= needed_size { Some(s) } else { None })
        .collect::<Vec<_>>();
    dbg!(&result);
    result.sort_unstable();
    dbg!(&result);
    println!("min = {}", result.first().expect("at least one"));
}
