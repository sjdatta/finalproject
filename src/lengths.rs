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
