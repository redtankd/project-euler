extern crate project_euler;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

use project_euler::*;

type MyType = i32;

fn main() {
    let (line_count, lines) = read_file("resource/00018.txt");

    let t = start_timer();

    println!("\nsolution for problem 18:\n");
    let (total, route) = s1(line_count, lines);
    println!("the maximum total = {}", total);
    print!("the route = ");
    for number in route {
        print!("{:?},", number);
    }
    println!("\n");

    stop_timer(t);

    //-------------------------------------------------

    let (line_count, lines) = read_file("resource/00067.txt");

    let t = start_timer();

    println!("\nsolution for problem 67:\n");
    let (total, route) = s1(line_count, lines);
    println!("the maximum total = {}", total);
    print!("the route = ");
    for number in route {
        print!("{:?},", number);
    }
    println!("\n");

    stop_timer(t);
}

// read the data from a file to a 2-dimesion array
fn read_file(name: &str) -> (usize, Vec<Vec<MyType>>) {
    let mut line_count: usize = 0;
    let mut lines = Vec::new();

    let f = File::open(name).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut vec = Vec::new();

        for number in line.split_whitespace() {
            vec.push(MyType::from_str(number).unwrap());
        }

        line_count += 1;
        lines.push(vec);
    }

    (line_count, lines)
}

struct Node {
    total: MyType,
    route: Vec<MyType>,
} 

// search from bottom to top. 
// calculate all possible total for every element and find the maximum total.
fn s1(line_count: usize, data_lines: Vec<Vec<MyType>>) 
    -> (MyType, Vec<MyType>) {

    let mut last_line = Vec::<Node>::new();
    let mut current_line = Vec::<Node>::new();

    // search every element from bottom to top
    for i in (0..line_count).rev() {
        current_line.clear();
        let ref data_line = data_lines[i];

        // from left to right
        for j in 0..i+1 {
            let mut node = Node { 
                total: data_line[j],
                route: vec![data_line[j]]
            };

            // every element but the last line has two side to move down.
            // so find the side whose total is bigger.
            if i < line_count-1 {
                let max_index: usize = 
                    if last_line[j].total > last_line[j+1].total { j } 
                    else { j+1 };

                // calculate the maximum for the current element
                node.total += last_line[max_index].total;
                // record the route
                node.route.append(&mut last_line[max_index].route.clone());
            }

            current_line.push(node);
        }

        last_line.clear();
        last_line.append(&mut current_line);
    }

    (last_line[0].total, last_line[0].route.clone())
}

#[test]
fn s1_test() {
    let (line_count, lines) = read_file("src/resource/00018.txt");
    assert_eq!(1074, s1(line_count, lines).0);

    let (line_count, lines) = read_file("src/resource/00067.txt");
    assert_eq!(7273, s1(line_count, lines).0);
}