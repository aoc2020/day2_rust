use std::fs::File;
use std::io::{self, Lines, BufReader, BufRead};
use std::path::Path;
use regex::Regex;


fn main() {
    println!("Hello, world!");
    let pws = read_pwlines();
    let valids1 = pws.iter().filter(|pw| pw.valid1()).count();
    let valids2 = pws.iter().filter(|pw| pw.valid2()).count();
    println!("Answer 1 : {}", valids1);
    println!("Answer 2 : {}", valids2);
}

struct PasswordLine {
    first: usize,
    second: usize,
    char: char,
    password: String,
}

impl PasswordLine {
    fn new(line: &str) -> PasswordLine {
        let regex = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
        let rmatch = regex.captures(line).unwrap();
        let min: usize = rmatch.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = rmatch.get(2).unwrap().as_str().parse().unwrap();
        let c: char = rmatch.get(3).unwrap().as_str().parse().unwrap();
        let pw: &str = rmatch.get(4).unwrap().as_str();
        return PasswordLine { first: min, second: max, char: c, password: pw.to_string() };
    }

    fn valid1(&self) -> bool {
        let count = self.password.chars().filter(|c| c.clone() == self.char).count();
        let isValid = count >= self.first && count <= self.second;
        return isValid;
    }

    fn valid2(&self) -> bool {
        let first = self.password.as_bytes()[self.first -1] as char;
        let second = self.password.as_bytes()[self.second -1] as char;
        let firstOk = first == self.char && second != self.char;
        let secondOk = first != self.char && second == self.char;
        return firstOk || secondOk;
    }
}

fn read_pwlines() -> Vec<PasswordLine> {
    let mut pwLines: Vec<PasswordLine> = vec!();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(s) = line {
                let pwLine = PasswordLine::new(&s);
                pwLines.push(pwLine);
            }
        }
    }
    return pwLines;
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}