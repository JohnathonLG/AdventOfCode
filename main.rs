/******************************************************************************

                            Online Rust Compiler.
                Code, Compile, Run and Debug Rust program online.
Write your code in this editor and press "Run" button to execute it.

*******************************************************************************/
use std::fs;
use std::collections::HashSet;

fn charToVal(ch: char) -> i32 {
    if ch.is_ascii_lowercase() {
        return (ch as i32) - 96;
    } else {
        return (ch as i32) - 38;
    }
}

fn day3() {
    let data = fs::read_to_string("./day3.txt").expect("Unable to read file");
    let mut sum = 0;
    let mut itr = data.lines();
    while let Some(line1) = itr.next() {
        let line2 = itr.next().unwrap();
        let line3 = itr.next().unwrap();
        let firstCharSet: HashSet<char> = line1.chars().collect();
        let secondCharSet: HashSet<char>  = line2.chars().collect();
        let thirdCharSet: HashSet<char> = line3.chars().collect();
        for ch in firstCharSet {
            if secondCharSet.contains(&ch) && thirdCharSet.contains(&ch) {
                println!("{}", ch);
                sum += charToVal(ch);
            }
        }
        
    }
    println!("{}", sum);
}

fn day4() {
    let data = fs::read_to_string("./day4.txt").expect("Unable to read file");
    let mut count = 0;
    for line in data.lines() {
        let parts: Vec<&str> = line.split(&['-', ','][..]).collect();
        let a1: i32 = parts[0].parse().unwrap();
        let b1: i32 = parts[1].parse().unwrap();
        let a2: i32 = parts[2].parse().unwrap();
        let b2: i32 = parts[3].parse().unwrap();
        if ((a1 >= a2 && a1 <= b2) || (a1 <= a2 && a2 <= b1) || (b1 <= b2 && b1 >= a2) || (b1 >= b2 && b2 >= a1)) {
            count += 1;
        }
    }
    println!("{}", count);
}
fn main() {
    day4();
}
