//Jesse A. Jones
//5 Sep, 2023
//Project 1: Determinant Calculator
//Version: 1.11

use std::io::Read;

//Given a reference to the matrix data, prints out a debug display 
// of the current matrix. 
fn _debug_print_matrix(mtrx: &Vec<Vec<f64>>) -> (){
    println!("Matrix is {} long and contains elements: ", mtrx.len());
    //Prints out special debug display of matrix.
    for row in mtrx.iter(){
        for el in row.iter(){
            print!(" {} |", el);
        }
        println!("");
    }
}

//Takes in an assumed valid string 
// and generates an NXN matrix based on it.
fn string_to_nxn_matrix(str: String) -> (Vec<Vec<f64>>, usize){
    //Converts input matrix string into a list 
    // of floats to be turned into matrix.
    let elements: Vec<f64> = str
        .split_whitespace()
        .map(|el| el.parse::<f64>()
            .expect("INVALID NUMBER"))
        .collect();

    let length = elements[0] as usize;

    let mut matrix: Vec<Vec<f64>> = Vec::new();

    //Iterates through floating point list and builds matrix.
    for i in 0..length{
        let mut matrix_row: Vec<f64> = Vec::new();
        
        //Fills out matrix row from parsed numbers.
        for j in 0..length{
            matrix_row.push(elements[(i * length) + j + 1]);
        }

        //Adds matrix row to whole matrix.
        matrix.push(matrix_row);
    }
    (matrix, length as usize)
}

//Calculates the determinat of the input matrix 
// and its input length and returns it.
fn determinant_calc(mut matrix: Vec<Vec<f64>>, length: usize) -> f64{
    //Used in determining the final sign of the determinant.
    let plus_minus_arr: [f64; 2] = [1.0, -1.0];
    let mut swap_count: usize = 0;

    let mut determinant: f64 = 1.0;

    //Row reduces the input matrix and calculates the determinat as it does so.
    for i in 0..length{

        //If the potential pivot spot is a zero, 
        // it scans down the column to find a non-zero to swap in 
        // to make a non-zero pivot, 
        // if no non-zero value is found, return 0.0 since 
        // it would result in a zero determinant. 
        if matrix[i][i] == 0.0f64{
            let mut new_pivot_found = false;
            //Finds successor pivot.
            for j in i..length{
                if matrix[j][i] != 0.0f64{
                    matrix.swap(i, j);
                    new_pivot_found = true;
                    swap_count += 1;
                }  
            }
            if !new_pivot_found{return 0.0f64;}
            
        }

        //Scales a by dividing it by a scalar based on itself. 
        // This scalar factor is then multiplied 
        // to the determinant accumulator. 
        let div_val: f64 = matrix[i][i];
        if div_val != 1.0f64{
            determinant *= div_val;
            for j in i..length{
                matrix[i][j] /= div_val;
            }
        }

        //Goes down a given column and finds any rival pivot points, 
        // if any are found, subtraction is done to eliminate 
        // them via subtracting one row from another scaled by determined factor.
        for j in (i + 1)..length{
            if matrix[j][i] != 0.0f64{
                let ratio: f64 = matrix[j][i];

                //Adds the pivot row times a negative factor 
                // of the rival pivot to cancel it out.
                for k in i..length{
                    matrix[j][k] += matrix[i][k] * ratio * (-1.0f64);
                } 
            }
        }

    }

    //Calculates final determinant and returns it.
    determinant * plus_minus_arr[swap_count % 2]
}

//Performs IO and calls function to calculate determinant.
fn main(){
    //Reads in input matrix size and matrix elements.
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input).expect("READING FAILED");

    //Converts input string to matrix representation.
    let matrix_stuff = string_to_nxn_matrix(input);

    let result = determinant_calc(matrix_stuff.0, matrix_stuff.1);

    println!("{}", result);

}