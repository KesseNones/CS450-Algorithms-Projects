# CS 450 Fall 2023 Algorithms Projects Implementations by Jesse Jones

## Overview
These are the projects I implemented in Fall of 2023 at university for my algorithms class. All projects are implemented in Rust and demonstrate my knowledge of Rust gained in making these projects.

If you want to play with these yourself, you'll need at least rustc
installed to compile the rust programs. Having a unix-based system
terminal also helps a lot.

## Disclaimer
These projects were created for WSUV's algorithms class, so if you are
a student in that class, leave, now. If you use these to cheat I will
find you and I will put hairs in your shirt when you're not looking,
making you extremely itchy. So, don't cheat, m'kay?

## Contents
Contained in this repo are five projects each demonstrating my knowledge
of Rust and algorithm design. 

All five of these projects are compiled the same way:
```
rustc -C opt-level=3 main.rs
```
This replicates the `cargo build --release` command I used
for building these projects when they were still in crates.

### first_project
A matrix multiplier algorithm done as efficiently
as possible.

#### Running
Compile and then run like so:
```
./main < INPUT_TEXT_FILE
```
Where `INPUT_TEXT_FILE` is a text file that has one line 
representing the dimensions of a matrix with the following lines
following being lines of the n x n matrix.

### second_project
The project involved with beating Rust's standard sorting function by 
creating a custom sorting function that utilized Counting Sort to
vastly speed up sorting the high scores input data.

#### Running
Compile and then run like so:
```
./main <standard|custom> < INPUT_PLAYER_DATA > OUTPUT_FILE
```
The argument of `<standard|custom>` is used to determine if Rust's standard
sorting library is used to sort the data or the custom made, quicksort
counting sort hybrid is used to sort the data.

The argument `INPUT_PLAYER_DATA` is the input file that contains the player
data to be parsed and sorted. Look at data/players.dat or 
data/players_sample.dat for examples of the format. 

The final argument is an indirection of stdout to an output file 
which represents the scores of each category in sorted order.
Look at data/players_sample_out.txt or players_out2.txt for examples.

### third_project
An implementation of a Scapegoat Tree and AVL Tree using an arena as a quasi-heap, in order to speed the program up and get around Rust's ownership and borrowing rules. This was easily the hardest project to implement of the five.

#### Running
Compile and then run like so:
```
./main <scapegoat|avl> GRIEFER_DATA_FILE < USER_QUERY_INPUT_LINES > OUTPUT_FILE
```
The first argument determines if a scapegoat tree or avl tree is used
to store the data from `GRIEFER_DATA_FILE`. The third argument
specifies the input lines with each line being a user to query
to see if they are in the griefer data structure tree.
The last argument is an optional output file to redirect the output since
it can be chunky if querying a lot of users.

### fourth_project
An example of using dynamic programming to make the classic player knapsack problem get solved in linear time (O(n)).

#### Running
Compile and run like so:
```
./main < INPUT_ITEMS
```
Where `INPUT_ITEMS` is one of the input data files containing a list
of items to take, their weights, and values.

### fifth_project
The project is an implementation of the A-Star graph searching algorithm.

#### Running
Compile and run as such:
```
./main < INPUT_MAP
```
Where `INPUT_MAP` is a text map for the A-Star algorithm to navigate.
If you want to try this yourself, use one of the existing maps found in the
`data` folder of `fifth_project`.

## Conclusion
While none of this is anything amazing, it is a good showcase of some work I've done with the Rust programming lanugage. 

