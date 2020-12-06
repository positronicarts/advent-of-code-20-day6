use clap::Clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clap)]
struct Opts {
    part: i32,
    input: String,
}

#[derive(Default)]
struct Group {
    groups: Vec<String>,
}

impl Group {
    fn get_count_pt1(&self) -> usize {
        let mut letters = std::collections::HashSet::<char>::new();
        for group in &self.groups {
            for answer in group.chars() {
                letters.insert(answer);
            }
        }
        letters.len()
    }

    fn get_count_pt2(&self) -> usize {
        let mut letters: std::collections::HashSet<char> = self.groups[0].chars().collect();
        for group in &self.groups {
            let answer_cs = group.chars().collect::<Vec<char>>();
            let mut remove_cs = Vec::<char>::new();
            for letter in &letters {
                if !answer_cs.contains(&letter) {
                    remove_cs.push(*letter);
                }
            }

            for letter in remove_cs {
                letters.remove(&letter);
            }
        }
        letters.len()
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = get_groups(opts.input);
    if opts.part == 1 {
        let count: usize = groups.iter().map(Group::get_count_pt1).sum();
        println!("Count {}", count);
    } else {
        let count: usize = groups.iter().map(Group::get_count_pt2).sum();
        println!("Count {}", count);
    }
}

fn get_groups(filename: String) -> Vec<Group> {
    let mut groups = Vec::<Group>::new();

    let mut current_group = Group::default();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_as_string) = line {
                if line_as_string == "" {
                    groups.push(current_group);
                    current_group = Group::default();
                } else {
                    current_group.groups.push(line_as_string);
                };
            }
        }
    }

    if !current_group.groups.is_empty() {
        groups.push(current_group);
    }

    groups
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
