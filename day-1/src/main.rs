use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;

fn find_double() {
    let mut total = 0;
    let mut totalVec = vec![0];
    loop {
        let f = File::open("input.txt").expect("Error while opening file");
        let mut file = BufReader::new(&f);
        
        for line in file.lines() {
            let l = line.expect("Error while unwrapping line");
            match &l[0..1] {
                "+" => {
                    total += &l[1..].parse::<i64>().expect("Error while parsing addition");

                    if totalVec.contains(&total) {
                        println!("First repeated frequency is : {}", total);
                        return
                    }

                    totalVec.push(total);
                },
                "-" => {
                    total -= &l[1..].parse::<i64>().expect("Error while parsing subtraction");

                    if totalVec.contains(&total) {
                        println!("First repeated frequency is : {}", total);
                        return
                    }

                    totalVec.push(total);
                },
                _ => unimplemented!(),
            }
        }
    }

}

fn find_total() {
    let f = File::open("input.txt").expect("Error while opening file");
    let mut file = BufReader::new(&f);
    
    let mut total = 0;

    for line in file.lines() {
        let l = line.expect("Error while unwrapping line");
        match &l[0..1] {
            "+" => {
                total += &l[1..].parse::<i64>().expect("Error while parsing addition");
            },
            "-" => {
                total -= &l[1..].parse::<i64>().expect("Error while parsing subtraction");
            },
            _ => unimplemented!(),
        }
    }
    println!("Final Frequency is : {}", total);
}

fn main() {
    find_total();
    find_double();
}
