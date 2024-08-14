//Jesse A. Jones
//14 Nov, 2023
//CS 450
//Project V: Auto Loot
//Version: 1.0.1

/*
    IMPORTANT INFO:
        -I am using A* for my algorithm which is faster than Dijkstra's!

*/

use std::io::Read;
use std::io::{Write, BufWriter};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

struct Node{
    appearance: char,
    node_cost: f64,
    net_cost: f64
}

impl Node{
    fn new(tile_chr: char) -> Self{
        //Assigns the appropriate cost based on the given character.
        let cost: f64 = match tile_chr{
            '.' => 1.0,
            ',' => 2.0,
            'o' => 3.0,
            '=' => 50.0,
            '1' | '3' | '0'  => f64::MAX,
            '2' => 0.0,
            _ => panic!("INVALID CHAR DETECTED"),
        };

        Node{
            appearance: tile_chr, 
            node_cost: cost,
            net_cost: 0f64
        }
    }
}

//Used in priorty queue.
struct QueueNode{
    cost_func: f64,
    row: usize,
    col: usize,
}

//Makes it so the QueueNode struct 
// can be used in the priority queue as a minheap.
impl PartialEq for QueueNode {
    fn eq(&self, other_item: &Self) -> bool {
        self.cost_func.eq(&other_item.cost_func)
    }
}
impl Eq for QueueNode {}
impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other_item: &Self) -> Option<Ordering> {
        other_item.cost_func.partial_cmp(&self.cost_func)
    }
}
impl Ord for QueueNode {
    fn cmp(&self, other_item: &Self) -> Ordering {
        self.cost_func
            .partial_cmp(&other_item.cost_func)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

struct Graph{
    height: usize,
    width: usize,
    nodes: Vec<Vec<Node>>,
    player_coords: (usize, usize),
    goal_coords: (usize, usize),
}

impl Graph{
    //Parses input data into a proper graph.
    fn new(tile_str: String) -> Self{
        let rows: Vec<&str> = tile_str
            .lines()
            .collect();

        let height = rows.len();
        let mut nodes: Vec<Vec<Node>> = Vec::with_capacity(height);
        let mut width = 0;
        let mut plr_tup = (usize::MAX, usize::MAX);
        let mut goal_tup = (usize::MAX, usize::MAX);

        //Fills out the data structure.
        for i in 0..(rows.len()){
            let chrs: Vec<char> = rows[i]
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            
            width = chrs.len();
            
            let mut node_row: Vec<Node> = Vec::with_capacity(width); 

            //Fills out row of graph.
            for j in 0..width{
                //If character is the starting player, 
                // update the coordinates for the player to reflect that.
                if chrs[j] == '0'{
                    plr_tup.0 = i;
                    plr_tup.1 = j;
                }

                //If the character is the goal, 
                // update the coords to save the position.
                if chrs[j] == '2'{
                    goal_tup.0 = i;
                    goal_tup.1 = j;
                }

                //Adds node to row.
                node_row.push(Node::new(chrs[j]));
            }

            //Adds row to graph.
            nodes.push(node_row);
        }

        Graph{
            height: height,
            width: width,
            nodes: nodes,
            player_coords: plr_tup,
            goal_coords: goal_tup,
        }
    }

    //Calculates the magnitude of the distance 
    // from the current node position to the goal.
    fn heuristic(&self, row: usize, col: usize) -> f64{
        let row_delta = (self.goal_coords.0 as f64) - (row as f64);
        let col_delta = (self.goal_coords.1 as f64) - (col as f64);

        f64::sqrt((row_delta * row_delta) + (col_delta * col_delta))
    }

    //Checks if the set of coordinates fits 
    // within the range of the graph scale.
    fn is_valid_coords(&self, row: isize, col: isize) -> bool{
        row < (self.height as isize) && col < (self.width as isize) && row > -1 && col > -1
    }

    //Given a coordinate set, finds the valid successors to a given node.
    fn find_successors(&self, explored: &HashMap<(usize, usize), f64>, 
            curr_row: isize, curr_col: isize) -> Vec<(usize, usize)>{
        
        let mut successors: Vec<(usize, usize)> = Vec::with_capacity(8);

        //All possible directions the player can move.
        let shifts: [(isize, isize); 8] = [
            (1, 0), (-1, 0), (0, -1), (0, 1),
            (1, -1), (1, 1), (-1, -1), (-1, 1)
        ];

        //Finds all valid successors and pushes them to list.
        for delta in shifts{
            //If the successor is in the coordinate range 
            // and it hasn't been seen before, 
            // add it to the list of valid successors.
            if self.is_valid_coords(curr_row + delta.0, curr_col + delta.1) 
                && 
                !explored.contains_key(&((curr_row + delta.0) as usize, 
                        (curr_col + delta.1) as usize)) {
                
                successors.push(((curr_row + delta.0) as usize, 
                    (curr_col + delta.1) as usize));   
            }
        }

        successors
    }

    //Finds the optimal path from the player to the goal, 
    // returns a vec of indicies in the graph representing the path.
    fn a_star_pathfind(&mut self) -> Vec<(usize, usize)>{
        //Used to track newly found nodes 
        // and order them by cost.
        let mut horizon: BinaryHeap<QueueNode> = BinaryHeap::new();

        let mut found_nodes: HashMap<(usize, usize), f64> = HashMap::new();

        //Stories previous node relative to current node.
        let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        let player_row = self.player_coords.0;
        let player_col = self.player_coords.1;

        //Pushes root to queue.
        horizon.push(QueueNode{cost_func: 0.0 + self.heuristic(player_row, player_col), 
            row: player_row, col: player_col});

        //Runs pathfinding algorithm until goal is found.
        while !horizon.is_empty(){
            let curr_node: QueueNode = horizon.pop().expect("UNEXPECTED FAILURE");
            
            //If current node is the goal, stop running.
            if curr_node.row == self.goal_coords.0 && curr_node.col == self.goal_coords.1{
                break;
            }

            found_nodes.insert((curr_node.row, curr_node.col), curr_node.cost_func);

            let successors = self.find_successors(
                &found_nodes, curr_node.row as isize, curr_node.col as isize
            );

            //Goes through all potential successors 
            // and adds them to the queue if needed.
            for successor in successors{
                //If the successor is diagonal from the current node, 
                // the multiplication of 1.5 will need to occur 
                // to the node cost of the successor node.
                let row_delta = (successor.0 as isize) - 
                                (curr_node.row as isize);
                let col_delta = (successor.1 as isize) - 
                                (curr_node.col as isize);
                let diagonal_factor: f64; 
                if row_delta != 0 && col_delta != 0{
                    diagonal_factor = 1.5;
                }else{
                    diagonal_factor = 1.0;
                }

                //New cost based on the successor node cost, 
                // current node net cost, and diagonal factor.
                let updated_cost = (self.nodes[successor.0][successor.1].node_cost * diagonal_factor) + 
                    self.nodes[curr_node.row][curr_node.col].net_cost;

                //Updates cost if cost hasn't been found 
                // or if a better cost has been found.                
                if self.nodes[successor.0][successor.1].net_cost == 0.0 || 
                    self.nodes[successor.0][successor.1].net_cost > updated_cost{
                    
                    //Sets successor node's cost to the updated value.
                    self.nodes[successor.0][successor.1].net_cost = updated_cost;

                    //Adds to priority queue.
                    horizon.push(QueueNode{
                        cost_func: updated_cost + 
                                    self.heuristic(successor.0, successor.1),
                        row: successor.0, col: successor.1
                    });

                    //Adds to hash for later path recall.
                    previous.insert((successor.0, successor.1), (curr_node.row, curr_node.col));

                }
            }
        }

        //Stories the cooridnates of the nodes involved in the best path.
        let mut path_indices: Vec<(usize, usize)> = Vec::new();

        let mut curr_row = self.goal_coords.0;
        let mut curr_col = self.goal_coords.1;

        //Builds up list of coords of best path nodes.
        loop{
            match previous.get(&(curr_row, curr_col)){
                Some((r, c)) => {
                    curr_row = *r; 
                    curr_col = *c;
                    path_indices.push((curr_row, curr_col));
                },
                None => {break;},
            }
        }
        path_indices
    }
}

fn main() {   
    //Reads in input map from stdin.
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("FAILED TO READ INPUT DATA");

    let mut graph = Graph::new(input);

    //Establishes buffer used to write as output.
    let mut out_buff: BufWriter<std::io::Stdout> = BufWriter::new(std::io::stdout());

    let index_path = graph.a_star_pathfind();

    let total_cost = graph.nodes[index_path[0].0][index_path[0].1].net_cost;

    //Replaces icons in map to indicate found path.
    for i in 0..(index_path.len() - 1){
        graph.nodes[index_path[i].0][index_path[i].1].appearance = '*';
    }

    //Prints out final map after path has been found.
    for row in graph.nodes.iter(){
        for i in 0..row.len(){
            write!(out_buff, "{}", row[i].appearance).expect("WRITING FAILED");
            if i < row.len() - 1{write!(out_buff, " ").expect("WRITING FAILED")}
        }
        writeln!(out_buff, "").expect("WRITING FAILED");
    }

    //Displays total cost when done.
    writeln!(out_buff, "Total cost: {}", total_cost).expect("WRITING FAILED");

}
