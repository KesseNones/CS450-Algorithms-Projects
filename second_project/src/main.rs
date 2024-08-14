//Jesse A. Jones
//28 Sep, 2023
//CS 450
//Project II: High Scores
//Version: 1.11

use std::io::Read;
use std::io::{Write, BufWriter};
use std::env;
use std::time::{Instant};

//Performs a partition of a sub-array following Hoare partition.
fn hoare_partition(unsorted: &mut Vec<u32>, low: usize, high: usize) -> usize{
    let pivot: usize = unsorted[(high + low) / 2] as usize;
    let mut left = low - 1;
    let mut right = high + 1;

    loop{
        //Moves arrow partitions.
        left += 1;
        while (unsorted[left] as usize) > pivot {left += 1}
        right -= 1;
        while (unsorted[right] as usize) < pivot {right -= 1}
        if left >= right {return right;}
        
        //Swaps two values.
        let old_left = unsorted[left];
        unsorted[left] = unsorted[right];
        unsorted[right] = old_left;
    }
}

//Performs in-place quicksort on input data in index range.
fn quicksort_in_place(unsorted: &mut Vec<u32>, low: usize, high: usize){
    if low < high{
        let new_pivot: usize = hoare_partition(unsorted, low, high);

        quicksort_in_place(unsorted, low, new_pivot);
        quicksort_in_place(unsorted, new_pivot + 1, high);
    }
}

//Performs counting sort on input data.
fn counting_sort(unsorted: &mut Vec<u32>, low: usize, high: usize){
    let mut count_arr: [u32; 10000] = [0; 10000];

    //Updates all number counts based on data.
    for i in low..high{
        count_arr[unsorted[i] as usize] += 1;
    }

    //Builds sorted version of the array in descending order.
    let mut index: i32 = 9999;
    let mut sorted_index: usize = low; 
    while index > -1{
        let count: usize = count_arr[index as usize] as usize;
        for _ in 0..count{
            unsorted[sorted_index] = index as u32;
            sorted_index += 1;
        }
        index -= 1;
    }
}

//Uses a combination of counting and quicksort 
// to sort the input player data.
fn custom_sort(mut unsorted: Vec<u32>) -> Vec<u32>{
    let size = unsorted.len();

    //Partitions unsorted column into numbers smaller and larger than 10 000.
    let mut big_index: usize = 0;
    for i in 0..size{
        if unsorted[i] >= 10000 && i != big_index{
            //Swaps bigger value with smaller value to parition the two.
            unsorted.swap(i, big_index);
            big_index += 1;

            //If next value is bigger, moves forward until index is smaller again.
            while unsorted[big_index] >= 10000{
                big_index += 1;
            }
        }
    }

    quicksort_in_place(&mut unsorted, 0, big_index - 1);
    counting_sort(&mut unsorted, big_index, size);

    unsorted
}

//Takes in some input data and writes to stdout buffer.
fn write_scores_to_buff(out_buff: &mut BufWriter<std::io::Stdout>, scores: &Vec<Vec<u32>>, 
    skill_name: &str, skill_index: usize){
    
    //Adds skill name.
    writeln!(out_buff, "{}", skill_name).expect("Failed to write!");

    //Builds up skill segment of string row by row.
    let size = scores[skill_index].len();
    for i in 0..size{
        writeln!(out_buff, "{}", scores[skill_index][i]).expect("Failed to write!");
    }
}

//Takes in the input player data string and converts it to 
// the desired player data data structure.
fn string_to_data(str: String) -> Vec<Vec<u32>>{
    //Converts input string to numbers.
    let nums: Vec<u32> = str
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Failed to write!"))
        .collect();

    let rows: usize = nums.len() / 5;

    //Used in building the final data structure to be sorted later.
    let mut data: Vec<Vec<u32>> = Vec::with_capacity(6);
    for _ in 0..6{
        data.push(Vec::with_capacity(rows));
    }


    //Iterates through numbers and builds data structure out of it.
    for row in 0..rows{
        let mut sum: u32 = 0;
        
        //Fills out each column of data structure.
        for col in 0..5{
            let index = (row * 5) + col;

            data[col].push(nums[index]);
            sum += nums[index];
        }
        data[5].push(sum);
        
    }

    data
}

fn main() {
    //Gets argv and argc
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    
    //Checks to make sure some kind of command line argument is given.
    if argc < 2{panic!("Insufficient args given!!!!!!!!");}

    //Checks to make sure given command arg is standard or custom.
    // Panics otherwise.
    let is_custom: bool;
    if argv[1] == "custom" || argv[1] == "standard"{
        is_custom = argv[1] == "custom";
    }else{
        panic!("Invalid command line argument given!");
    }

    //Reads in input player data.
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("FAILED TO READ FILE");

    //Will be used later for sorting and displaying the results.
    let mut player_data: Vec<Vec<u32>> = string_to_data(input);
    
    //Establishes buffer used to write sorted output.
    let mut out_buff: BufWriter<std::io::Stdout> = BufWriter::new(std::io::stdout());

    let mut total_time: u128 = 0;
    let name_arr = ["SKILL_BREAKDANCING", "SKILL_APICULTURE", "SKILL_BASKET", 
                "SKILL_XBASKET", "SKILL_SWORD", "TOTAL_XP"];

    //Sorts and adds result to print string for each column of player data.
    for i in 0..6{
        let start = Instant::now();
        
        //Sorts by custom sorting algorithim if specified, 
        // uses built in unstable sort otherwise.
        if is_custom{
            player_data[i] = custom_sort(std::mem::take(&mut player_data[i]));
        }else{
            player_data[i].sort_unstable_by(|a, b| b.cmp(a));
        }

        //Finds time taken for sort to occur in microseconds.
        let duration: u128 = start
            .elapsed()
            .as_micros();
        total_time += duration;

        //Adds sorted column and time taken to buffer.
        write_scores_to_buff(&mut out_buff, &player_data, 
            name_arr[i], i); 
        writeln!(out_buff, "time taken: {}\n", duration).expect("Failed to write!");       
    }

    //Writes total amount of time taken by all sorts.
    writeln!(out_buff, "total time taken: {}", total_time).expect("Failed to write!");

}
