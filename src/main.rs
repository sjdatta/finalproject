fn main() {
    let m = readfile("example.txt");
    println!("{:?}", m);
    println!("{:?}", lengths(m))

    
}


use std::fs::File;
use std::io::prelude::*;


fn readfile(path: &str) -> Vec<Vec<usize>> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        result.push((x, y));
    }

    let n = result.len();





    let mut graph_list : Vec<Vec<usize>> = vec![vec![];n];
    for (v,w) in result.iter() {
        graph_list[*v].push(*w);
        graph_list[*w].push(*v);
    
    };
    graph_list
    
}



//let's find some out some basic info about our dataset


//finding average number of products copurchased

fn lengths(v: Vec<Vec<usize>>) -> Vec<usize> {
    
    let mut tally: Vec<usize> = vec![0; v.len()];
    for i in 0..v.len() {
        let amount = v[i].len();
        tally[i] = amount;
        

    }

    

    let mut data: Vec<usize> = vec![0;3];
    //let's find the average
    let mut total = 0;
    let mut min = tally[0];
    let mut max = tally[0];

    for j in 0..tally.len() {
        total += tally[j];


        //find minimum
        if tally[j] < min {
            min = tally[j];
        }

        //find max
        if tally[j] > max {
            max = tally[j];
        }

    }

    data[0] = total/tally.len();
    data[1] = max;
    data[2] = min;
    return data;


    
}


