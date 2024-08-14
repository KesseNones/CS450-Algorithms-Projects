//Jesse A. Jones
//3 Nov, 2023
//CS 450
//Project IV: Auto Loot
//Version: 1.0.0

use std::io::Read;
use std::io::{Write, BufWriter};
use std::time::{Instant};

struct Item{
    name: String,
    weight: u32,
    value: u32
}

impl Item{
    fn new(name_str: &str, weight_str: &str, value_str: &str) -> Self{
        Item{
            name: name_str.to_string(),
            weight: weight_str.parse().expect("PARSING OF WEIGHT FAILED"),
            value: value_str.parse().expect("PARSING OF VALUE FAILED")
        }
    }
}

//Takes in a string and parses it into a vec 
// of items and the max weight of the player's storage.
fn parse_input_into_items(input_data: String) -> (Vec<Item>, u32){
    //Splits data up into tokens based on new lines.
    let tokens: Vec<&str> = input_data
        .lines()
        .collect();

    //Return data initialized.
    let max_weight: u32 = tokens[0].parse().expect("PARSING OF MAX WEIGHT FAILED");
    let mut items: Vec<Item> = Vec::with_capacity((tokens.len() / 3) + 1);

    //Used to make it easier to understand the pseudocode.
    items.push(Item::new("NULL ITEM", "0", "0") );

    //Inserts items into vec from data.
    for row in 1..(tokens.len()){
        let row_info: Vec<&str> = tokens[row]
            .split(|c| c == ';')
            .collect();
        items.push(Item::new(row_info[0], row_info[1], row_info[2]));
    }

    (items, max_weight)

}


//Creates the table to be used by the recursive knapsack function.
fn make_table(items: &Vec<Item>, 
        number_of_items: usize, total_capacity: usize) -> Vec<Vec<u32>>{
    //Creates initial table.
    let mut table: Vec<Vec<u32>> = Vec::with_capacity(number_of_items);
    for i in 0..number_of_items{
        table.push(Vec::with_capacity(total_capacity + 1));
        for _ in 0..=total_capacity{
            table[i].push(0);
        }
    }

    //Fills out the table based on the data of the items.
    for i in 1..number_of_items{
        for j in 1..=total_capacity{
            if items[i].weight > j as u32{
                table[i][j] = table[i - 1][j];
            }else{
                table[i][j] = std::cmp::max(
                    table[i - 1][j], 
                    table[i - 1][j - (items[i].weight as usize)] + items[i].value
                );
            }
        }
    }

    table
}

//Takes in the necessary inputs and solves for the best inventory.
// Returns a list of the best indicies.
fn knapsack_solve(items: &Vec<Item>, table: &Vec<Vec<u32>>, 
        i: usize, j: usize, solns: &mut Vec<usize>){
    //If there's nothing left to parse, returns.
    if i == 0{
        return;
    }
    //If the current item is larger than previous, 
    // add to solutions and recurse.
    //Otherwise just recurse.
    if table[i][j] > table[i - 1][j]{
        solns.push(i);
        knapsack_solve(items, table, 
                i - 1, j - ( items[i].weight as usize ), solns);
    }else{
        knapsack_solve(items, table, i - 1, j, solns);
    }
}
    
fn main() {
    let start = Instant::now();

    //Reads in input data from stdin.
    let mut input_data = String::new();
    std::io::stdin()
        .read_to_string(&mut input_data)
        .expect("FAILED TO READ INPUT DATA");

    //Establishes buffer used to write as output.
    let mut out_buff: BufWriter<std::io::Stdout> = BufWriter::new(std::io::stdout());
    
    //Fills out items from data and makes table.
    let items_and_capacity: (Vec<Item>, u32) = parse_input_into_items(input_data);
    let table = make_table(&items_and_capacity.0, 
        items_and_capacity.0.len(), items_and_capacity.1 as usize);

    //Finds the correct items to maximize 
    // the player's knapsack value for its total weight.
    let mut solutions: Vec<usize> = Vec::new();
    knapsack_solve(&items_and_capacity.0, &table, 
        items_and_capacity.0.len() - 1, items_and_capacity.1 as usize, &mut solutions);

    //Totals up the weight and value, 
    // and displays all the items 
    // of the ideal inventory.
    let mut total_weight: u32 = 0;
    let mut total_value: u32 = 0;
    for sol in solutions.iter().rev(){
        writeln!(out_buff, "{}, {}, {}\r", 
            items_and_capacity.0[*sol].name,
            items_and_capacity.0[*sol].weight,
            items_and_capacity.0[*sol].value
        ).expect("FAILED TO DISPLAY");
        total_weight += items_and_capacity.0[*sol].weight;
        total_value += items_and_capacity.0[*sol].value;
    }
    writeln!(out_buff, "final weight: {}\r", total_weight)
        .expect("WRITING FINAL WEIGHT FAILED!");
    writeln!(out_buff, "final value: {}\r", total_value)
        .expect("WRITING FINAL VALUE FAILED!");

    //Finds time taken for program to run.
    let duration: u128 = start
        .elapsed()
        .as_micros();

    //Writes out the final time taken for the program to run.
    writeln!(&mut out_buff, "time taken in microseconds: {}\r", duration)
        .expect("FAILED TO WRITE");
}
