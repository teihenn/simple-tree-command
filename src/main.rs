use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut target_dir = ".";
    if 2 <= args.len() {
        target_dir = &args[1];
    }

    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&target, 0);
}

fn tree(target: &path::PathBuf, level: isize) {
    let files = target.read_dir().expect("path not found");

    for ent in files {
        let path = ent.unwrap().path();

        for _ in 1..=level {
            print!("|    ");
        }

        let fname = path.file_name().unwrap().to_string_lossy();

        if path.is_dir() {
            println!("|-- <{}>", fname);
            tree(&path, level + 1);
            continue;
        }

        println!("|-- {}", fname);
    }
}
