use rand::prelude::*;
use std::env;
use std::fs;

fn get_value() -> Vec<String> {
    let items: String = fs::read_to_string("src/adjectives.txt").expect("Should have been able to read from file");
    let value: Vec<&str> = items.split("\r\n").collect();
    let c: Vec<String> = value.into_iter().map(|x| x.to_string()).collect();
    return c;
}

// <fixed/fixes/fix/added/adds/add/removed/removes/remove> <adjective> <bug/feature>
// version 1.0.0
const FIRST_PART: [&str;9] = ["fixed", "fixes", "fix", "added", "adds", "add", "removed", "removes", "remove"];
const THIRD_PART: [&str;4] = ["bug", "feature", "stuff", "things"];
const VERSION: &str = "1.0.0";

/*

fixed identical bug
removed feline feature
fix loyal bug

*/

fn print_parts(second_part: Vec<String>) {
    let mut rng: ThreadRng = rand::thread_rng();
    let (o1, o2, o3) = (rng.gen_range(0..9), rng.gen_range(0..second_part.len()), rng.gen_range(0..4));
    println!("{} {} {}", FIRST_PART[o1], second_part[o2], THIRD_PART[o3])
}

fn version() {
    println!("Version {VERSION}");
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let second_part: Vec<String> = get_value();

    if args.len() == 2 {
        match args[1].as_str()
        {
            "-v" => {version()},
            "--version" => {version()},
            _ => {},
        }
    }

    print_parts(second_part);
}
