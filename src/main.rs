fn main() {
    
    //let m = readfile("example.txt");
    //let f = lengths(m.clone());
    //println!("{:?}", f);
    //println!("{:?}", breadth(m));
    
    let m = finalfunction("example.txt");
    
    

    
}


use std::fs::File;
use std::io::prelude::*;
use std::io;



fn readfile(path: &str) -> Vec<(usize, usize)> {
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

    result

    
    
}



//let's find some out some basic info about our dataset


//finding average number of products copurchased

fn lengths(v: Vec<(usize, usize)>) -> Vec<usize> {
    //reorganizing the vector to make it easier
    let n = v.len();


    let mut graph_list : Vec<Vec<usize>> = vec![vec![];n];
    for (v,w) in v.iter() {
        graph_list[*v].push(*w);
        graph_list[*w].push(*v);
    
    }
    
    
    let mut tally: Vec<usize> = vec![0; graph_list.len()];
    for i in 0..graph_list.len() {
        let amount = graph_list[i].len();
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


    //average
    data[0] = total/tally.len();
    //max
    data[1] = max;
    //min
    data[2] = min;
    

    return data;


    
}


fn findmax(edges: Vec<(usize, usize)>) -> usize {
    let mut max = edges[0].1;
    for i in 0..edges.len() {
        if max < edges[i].0 {
            max = edges[i].0;
        } else if max < edges[i].1 {
            max = edges[i].1;
        }
    }

    max
}

type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}

// reverse direction of edges on a list
fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl Graph {
    fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
    fn create_undirected(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }
}

//let's do a breadth first search to find out how connected a specific node is to the other nodes

use std::collections::VecDeque;

fn breadth(edges: Vec<(usize, usize)>) -> Vec<Option<u32>> {

    let mut start = String::new();
    io::stdin().read_line(&mut start).expect("Failed to read line");
    let start = start.trim();
    let start: usize = start.parse().expect("Not a good number!");

    let n = findmax(edges.clone()) + 1;
    let graph = Graph::create_undirected(n, &edges);
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0);

    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { // consider all unprocessed neighbors of v
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
                
            }
        }
    }

    distance
}



fn finalfunction(path: &str) {
    let file = readfile(path);
    let data = lengths(file.clone());
    println!("Here's some information about your graph. The average connections is {}, the maximum connection is {}, 
    and the minimum number of connections is {}", data[0], data[1], data[2]);
    println!("Please input the node number that you want to see the distances to:");
    let distance = breadth(file);
    print!("vertex:distance");
    for v in 0..distance.len() {
    print!("   {}:{}",v,distance[v].unwrap());
    }


    

}


#[test]
//test to make sure our average/max/min function "lengths" works

fn testlengths() {
    let example = readfile("example.txt");
    let m = lengths(example);

    assert_eq!(m, [2, 5, 0], "test failed");

}
#[test]
fn testbreadth() {
    let example = readfile("example.txt");
    let m = breadth(example, 2);
    assert_eq!(m, [Some(2), Some(2), Some(0), Some(1), Some(2)], "test failed");
}

