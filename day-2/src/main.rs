use std::collections::btree_map::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::fs::File;

fn count_characters(line: &String) -> BTreeMap<char, u64>{
    let mut count = BTreeMap::new();

    for c in line.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    count
}

fn two_occurance(tally: &BTreeMap<char, u64>) -> bool {
    for (_, count) in tally {
        if count % 2 == 0 {
            return true
        }
    }

    false
}

fn three_occurance(tally: &BTreeMap<char, u64>) -> bool {
    for (_, count) in tally {
        if count % 3 == 0 {
            return true
        }
    }

    false
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn hash_line(line: &String, map : &mut HashMap<u64, String>) -> Vec<u64> {

    let mut hash_vec = vec![];

    for x in 0..line.len() {
        let mut local_string = line.clone();
        unsafe{
            local_string.as_bytes_mut()[x] = '*' as u8;
        }
        let hash = calculate_hash(&local_string);
        hash_vec.push(hash);
        map.insert(hash, local_string);
    }

    hash_vec
}

fn find_dup(vec : &Vec<u64>) -> u64 {
    let mut current = 0;
    for x in vec {
        if current == *x {
            return *x
        }
        current = *x;
    }

    0
}

fn main() {
    let f = File::open("input.txt").expect("Error while opening file");
    let file = BufReader::new(&f);

    let mut twos = 0;
    let mut threes = 0;

    let mut hash_list : Vec<u64> = vec![];
    let mut hash_map : HashMap<u64, String> = HashMap::new();

    for line in file.lines() {
        let line = line.expect("Error : Expectin String from file.lines()");
        let tally = count_characters(&line);

        if two_occurance(&tally) {
            twos += 1;
        }
        if three_occurance(&tally) {
            threes += 1;
        }

        hash_list.append(&mut hash_line(&line, &mut hash_map));
        
    }
    let total = twos * threes;
    println!("Checksum is : {}", total);

    hash_list.sort();

    let dup = find_dup(&hash_list);

    println!("Similar entry is : {}", hash_map[&dup]);

}
