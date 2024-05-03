fn main() {
    
}


fn read_file(path: &str) -> Vec<(usize, usize)> {
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
    return result;
}


//organizing the output from read_file into an easier to manipulate vector
fn organize(edges: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let n = findmax(edges.clone()) + 1;
    let mut graph_list : Vec<Vec<usize>> = vec![vec![];n];
    for (v,w) in edges.iter() {
        graph_list[*v].push(*w);
        graph_list[*w].push(*v);
    
    };
    graph_list
}


//finding the max vertex to know how long I need to make the vector
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


//let's find some out some basic info about our dataset


//finding average number of products copurchased

fn findaverage(v: Vec<Vec<usize>>) -> Vec<usize> {
    let n = findmax(v); 
    let mut tally = vec![0; v.len()]
    for i in 0..v.len() {
        let m = v[i];
        tally[m[0]] += 1;

    }

    tally
}