use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone)]
struct Path {
    sequence: Vec<String>,
}

impl Path {
    fn new() -> Self {
        Self { sequence: vec![] }
    }

    fn chdir(&mut self, dir: &str) {
        if dir == "/" {
            self.sequence = vec![];
        } else if dir == ".." {
            self.sequence.pop().unwrap();
        } else {
            self.sequence.push(dir.to_string());
        }
    }

    fn remove_front(&mut self) -> String {
        self.sequence.remove(0)
    }

    fn is_empty(&self) -> bool {
        self.sequence.is_empty()
    }
}

struct FileStructure {
    path: Path,
    size: u64,
    files: HashMap<String, u64>,
    directories: HashMap<String, Box<FileStructure>>,
}

impl FileStructure {
    fn new(path: Path) -> Self {
        Self {
            path,
            size: 0,
            files: HashMap::default(),
            directories: HashMap::default(),
        }
    }

    fn record_file(&mut self, loc: Path, name: &str, size: u64) {
        let mut fs = self;
        fs.size += size;
        for subpath in loc.sequence {
            fs = fs.directories.get_mut(&subpath).unwrap();
            fs.size += size;
        }
        fs.files.insert(name.to_string(), size);
    }

    fn record_directory(&mut self, loc: Path, name: &str) {
        let mut fs = self;
        for subpath in &loc.sequence {
            fs = fs.directories.get_mut(subpath).unwrap();
        }
        fs.directories
            .insert(name.to_string(), Box::new(FileStructure::new(loc)));
    }
}

fn read_input(input: &str) -> FileStructure {
    let lines: Vec<&str> = input.lines().collect();
    let mut current_path = Path::new();
    let mut res = FileStructure::new(current_path.clone());

    let mut i = 0;

    while i < lines.len() {
        let line = lines.get(i).unwrap();
        let command = line.strip_prefix("$ ").unwrap();
        if let Some(location) = command.strip_prefix("cd ") {
            current_path.chdir(location);
            i += 1;
        } else if command == "ls" {
            i += 1;
            while i < lines.len() && !lines.get(i).unwrap().starts_with('$') {
                let line = lines.get(i).unwrap();
                if let Some(name) = line.strip_prefix("dir ") {
                    res.record_directory(current_path.clone(), name);
                } else {
                    let parts: Vec<&str> = line.split(' ').collect();
                    let name = parts.get(1).unwrap();
                    let size = parts.first().unwrap();
                    res.record_file(
                        current_path.clone(),
                        name,
                        u64::from_str_radix(size, 10).unwrap(),
                    );
                }
                i += 1;
            }
        } else {
            panic!("command not recognised")
        }
    }
    res
}

fn solve_1(file_structure: &FileStructure) -> u64 {
    let mut sum = 0;
    let mut directories = vec![file_structure];
    while !directories.is_empty() {
        let fs = directories.pop().unwrap();
        if fs.size < 100_000 {
            sum += fs.size;
        }
        for f in fs.directories.values() {
            directories.push(f);
        }
    }
    sum
}

fn solve_2(file_structure: &FileStructure) -> u64 {
    let free_space = 70_000_000 - file_structure.size;
    let required = 30_000_000;
    let need_to_find = required - free_space;

    let mut res = u64::MAX;

    let mut directories = vec![file_structure];
    while !directories.is_empty() {
        let fs = directories.pop().unwrap();
        if fs.size >= need_to_find {
            res = res.min(fs.size);
        }
        for f in fs.directories.values() {
            directories.push(f);
        }
    }
    res
}

fn solve(input: &str) -> (u64, u64) {
    let file_structure = read_input(input);
    (solve_1(&file_structure), solve_2(&file_structure))
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        assert_eq!(solve(input), (95437, 24933642));
    }
}
