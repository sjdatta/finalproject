fn main() {
    //let m = finalfunction("example.txt");
    let m = readfile("example.txt");
    let s = lengths(m.clone());
    println!("{:?}", s);
     


    
}



use crate::lengths::lengths;
use std::fs::File;
use std::io::prelude::*;
use std::io;


//this function reads a file
fn readfile(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        result.push((x, y));
    }

    result

    
    
}



//this function takes in a vector of edges and computes several things about it
//1. the average number of connections, 2. the maximum number of connections, and 3. the minimum number of connections
//it returns a vector with this data in that order

//split this into a module because it can be used in many different ways
//not just for this project necessarily

pub mod lengths {
    use crate::findmax;
    pub fn lengths(v: Vec<(usize, usize)>) -> Vec<usize> {

        //reorganizing the vector to make the data analysis easier
        let n = findmax(v.clone()) + 1;
        let mut graph_list : Vec<Vec<usize>> = vec![vec![];n];
        for (v,w) in v.iter() {
            graph_list[*v].push(*w);
            graph_list[*w].push(*v);
        

        }
        //println!("{:?}", graph_list);
        //finding the total number of connections for finding the average
        let mut tally: Vec<usize> = vec![0; graph_list.len()];
        for i in 0..graph_list.len() {
            let amount = graph_list[i].len();
            tally[i] = amount;
            
    
        }
    
        
    
        let mut data: Vec<usize> = vec![0;3];
    
        //instantiate some variables for the average, minimum, and maximum
    
        let mut total = 0;
        let mut min = tally[0];

        let mut max = tally[0];
       // println!("{:?}", tally);
    
        for j in 0..tally.len() {
            //find total
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

}



//this function finds the maximum node in a vector of edges. This helps with making sure I create certain variables 
//(like graph variables in other function) to the proper length. 

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

//starting to create the framework for constructing a Graph, which we can then implement our breadth first searches on
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

//doing the breadth first search by getting user input and finding the distances from that node
fn userinput() -> usize {
    //get the user input

    let mut start = String::new();
    io::stdin().read_line(&mut start).expect("Failed to read line");
    let start = start.trim();
    let start: usize = start.parse().expect("Not a good number!");
    start

}

use std::collections::VecDeque;
fn breadth(edges: Vec<(usize, usize)>, start: usize) -> Vec<Option<u32>> {

    


    //create the graph
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


//I wanted a function that would organize all of my function and print statements in a nice, organized fashion.
fn finalfunction(path: &str) {
    let file = readfile(path);
    let data = lengths(file.clone());
    println!("Here's some information about your graph. The average connections a node has is {}, the maximum amount of connections is {}, 
    and the minimum number of connections is {}", data[0], data[1], data[2]);
    println!("Please input the node number that you want to see the distances to: ");
    let num = userinput();
    let distance = breadth(file, num);


    print!("vertex:distance");
    let mut finalvec = vec![vec![0, 0];distance.len()];

    for v in 0..distance.len() {
        if distance[v] == None{
            finalvec[v][0] = 0;
            finalvec[v][1] = v as u32;
            print!("   {}:{}",v, 0);
        } else {
            finalvec[v][0] = distance[v].unwrap();
            finalvec[v][1] = v as u32;
    
            print!("   {}:{}",v,distance[v].unwrap());

        }
        
    }
    println!();

    finalvec.sort();
    println!("the maximum distance is {} from node {}", finalvec[distance.len() -1][0], finalvec[distance.len() -1][1] );
    


}




#[test]
//test to make sure our average/max/min function "lengths" works
fn testlengths() {
    use crate::lengths::lengths;
    let example = readfile("example.txt");
    let m = lengths(example);

    assert_eq!(m, [2, 5, 1], "test failed");

}
#[test]
//test to make sure that the breadth function works as intended
fn testbreadth() {
    let example = readfile("example.txt");
    let m = breadth(example, 2);
    assert_eq!(m, [Some(2), Some(2), Some(0), Some(1), Some(2)], "test failed");
}

