//Jesse A. Jones
//30 Oct, 2023
//CS 450
//Project III: Griefer List
//Version: 1.2.0


/*
    IMPORTANT INFORMATION:
        THE CURRENT SUPPORTED TREES ARE:
            -Scapegoat
            -AVL

*/

use std::io::Read;
use std::io::{Write, BufWriter};
use std::env;
use std::time::{Instant};
use std::path::Path;
use std::fs::File;

//Calculation useful for scapegoat tree.
fn log32(n: usize) -> usize{
    let num = n as f64;
    (f64::log2(num) / f64::log2(3.0 / 2.0)) as usize
} 

struct NameContainer{
    names: Vec<String>,
    number_of_names: usize,
}

impl NameContainer{
    fn new(mut capacity: usize) -> Self{
        if capacity == 0{
            capacity = 64;
        }
        NameContainer{
            names: Vec::with_capacity(capacity),
            number_of_names: 0,
        }
    }

    //Inserts potentially a new string and inserts 
    // it to vector or just spits out a pointer.
    fn insert(&mut self, name: &str) -> usize{
        //Inserts if there's no strings to compare to.
        if self.number_of_names == 0{
            self.names.push(name.to_string());
            self.number_of_names += 1;
            return self.number_of_names - 1;
        }

        //Inserts if insertion name differs 
        // from name at the top of the name stack.
        if self.names[self.number_of_names - 1] != name{
            self.names.push(name.to_string());
            self.number_of_names += 1;
            return self.number_of_names - 1;

        }
        
        self.number_of_names - 1
    }
}

//Used in the scapegoat tree.
struct ScapeNode {
    player_name_index: usize,
    sids: Vec<usize>,
    time_banned: usize,
    left: i32,
    right: i32,
}

impl ScapeNode{
    //Creates new scapegoat node based on input.
    fn new(name_index: usize, banned: usize, sid: usize) -> Self{
        let mut node = ScapeNode{
            player_name_index: name_index, 
            sids: Vec::new(),
            time_banned: banned,
            left: -1, 
            right: -1,
        };
        node.sids.push(sid);
        node
    }
}

struct ScapeTree{
    nodes_in_tree: usize,
    root_index: usize,
    data: Vec<ScapeNode>,
    name_store: NameContainer,
    visited_nodes: Vec<(usize, usize)>,
    flattened_nodes: Vec<usize>
}

impl ScapeTree{
    //Makes new scapegoat tree with input capacity.
    fn new(mut capacity: usize) -> Self{
        if capacity == 0 {capacity = 64;}
        ScapeTree{
            nodes_in_tree: 0,
            root_index: 0,
            data: Vec::with_capacity(capacity),
            name_store: NameContainer::new(capacity),
            visited_nodes: Vec::with_capacity(capacity),
            flattened_nodes: Vec::with_capacity(capacity)
        }
    }

    //Sets the appropriate child node index for the children.
    //Returns boolean based on if insertion happened or not, 
    // final location before indexing finishes, and if a rebuild needs to occur.
    fn _set_child(&mut self, name_index: usize, sid: usize,
            insert_node_index: usize, curr_index: usize, depth: usize) -> (bool, usize, bool){

        //If inserting the root, no need to set children.
        if insert_node_index == self.root_index{return (false, curr_index, false);}
        
        //Stops insertion if node is already in tree.
        if &self.name_store.names[name_index] 
                == 
            &self.name_store.names[self.data[curr_index].player_name_index]
            {return (false, curr_index, false);}

        //Adds visited node to list.
        self.visited_nodes.push((curr_index, 0));

        //Adds insertion index at appropriate mark if node exists and doesn't otherwise.
        if &self.name_store.names[name_index] 
                < &self.name_store.names[self.data[curr_index].player_name_index]{
            
            //If empty child index found, set it to index 
            // of recently inserted node,
            // otherwise, recurse.
            if self.data[curr_index].left == -1{
                self.data[curr_index].left = insert_node_index as i32;
                let alpha_height = log32(self.nodes_in_tree + 1);
                return (true, (self.data[curr_index].left as usize), (depth + 1) > alpha_height);
            }else{
                return self._set_child(name_index, sid, insert_node_index, 
                    self.data[curr_index].left as usize, depth + 1);
            }
        }else{
            //If empty child index found, set it to index 
            // of recently inserted node,
            // otherwise, recurse.
            if self.data[curr_index].right == -1{
                self.data[curr_index].right = insert_node_index as i32;
                
                let alpha_height = log32(self.nodes_in_tree + 1);
                
                return (true, (self.data[curr_index].right as usize), (depth + 1) > alpha_height);
            }else{
                return self._set_child(name_index, sid, insert_node_index, 
                    self.data[curr_index].right as usize, depth + 1);
            }
        }

    }

    //Recursively calculates size of a sub tree.
    fn size(&self, index: i32) -> usize{
        if index == -1{
            return 0;
        }
        let i = index as usize;

        return 1 + self.size(self.data[i].left) + self.size(self.data[i].right); 
    }

    //Walks back up tree and finds scapegoat node index.
    fn _find_scapegoat(&mut self, traverse_index: usize, 
            mut found_scape: i32, mut found_scape_index: usize) -> (usize, usize){
        //If root reached and no other nodes 
        // were chosen beforehand, the root is the scapegoat node.
        if traverse_index == 0{
            if found_scape != -1{
                return (found_scape as usize, found_scape_index as usize);
            }else{
                return (self.visited_nodes[traverse_index].0, traverse_index);
            }
        }

        //Condition handles leaf node case.
        if (traverse_index + 1) != self.visited_nodes.len(){
            let parent_index: usize = self.visited_nodes[traverse_index - 1].0;
            let parent_left_index: i32 = self.data[parent_index].left;
            let parent_right_index: i32 = self.data[parent_index].right;

            //Determines index of sibling of current traversal node.
            let sibling_index: i32;
            if (self.visited_nodes[traverse_index].0 as i32) == parent_left_index{
                sibling_index = parent_right_index;
            }else{
                sibling_index = parent_left_index;
            }

            //Finds size of current sub-tree.
            let subtree_size = 
                1 + self.visited_nodes[traverse_index + 1].1 
                + self.size(sibling_index);
            self.visited_nodes[traverse_index].1 = subtree_size;

            //Used in checking for a scapegoat node.
            let complement = self.visited_nodes.len() - traverse_index;
            let subtree_alpha_height = log32(subtree_size);

            //Updates found scapegoat node index.
            if complement > subtree_alpha_height{
                found_scape = self.visited_nodes[traverse_index].0 as i32;
                found_scape_index = traverse_index;
            }

            return self._find_scapegoat(traverse_index - 1, found_scape, found_scape_index);

        }else{
            //Sets inserted node size to 1, 
            // due to it being a leaf and recurses up a layer.
            self.visited_nodes[traverse_index].1 = 1;
            return self._find_scapegoat(traverse_index - 1, found_scape, found_scape_index);
        }
    }

    //Performs in-order traversal on a sub-tree and flattens it.
    fn _flatten(&mut self, index: i32){
        if index == -1{ 
            return; 
        }

        let i = index as usize;

        //Sets children of given node back to "null" 
        // and saves the old values so traversal can continue.
        let left = self.data[i].left;
        let right = self.data[i].right;
        self.data[i].left = -1;
        self.data[i].right = -1;

        //Adds flattened node to list and performs needed recursion.
        self._flatten(left);
        self.flattened_nodes.push(i);
        self._flatten(right);
    }

    //Recursively inserts all children to root of ideal subtree.
    fn _ideal_insert(&mut self, low: usize, high: usize) -> i32{
        //Stops if range is too close together.
        if low >= high {
            return -1;
        }

        //Finds median of subtree to act as the subtree's root.
        let delta = high - low;
        let median = (delta / 2) + low;

        let subtree_root = self.flattened_nodes[median];

        //Sets children of sub-tree root via recursion.
        self.data[subtree_root].left = self._ideal_insert(low, median);
        self.data[subtree_root].right = self._ideal_insert(median + 1, high);

        //Returns root of subtree.
        return subtree_root as i32;

    }

    //Performs a subtree rebuild, taking 
    // in the subtree root and the subtree root's parent.
    fn _tree_rebuild(&mut self, subtree_parent_index: i32, subtree_root_index: usize){
        self._flatten(subtree_root_index as i32);

        //Rebuilds sub-tree using flattened nodes.
        let rebuild_root = self._ideal_insert(0, self.flattened_nodes.len()); 
        
        //Sets up the appropriate parent pointers.
        if subtree_parent_index == -1{
            //Tree's root replaced if entire tree is rebuilt.
            self.root_index = rebuild_root as usize;
        }else{
            //Sets root of rebuilt tree to replace the former child in its correct spot.
            if subtree_root_index == self.data[subtree_parent_index as usize].left as usize{
                self.data[subtree_parent_index as usize].left = rebuild_root;
            }else{
                self.data[subtree_parent_index as usize].right = rebuild_root;
            }

        }

        self.flattened_nodes.clear();

    }

    //Inserts player data into scapegoat tree.
    fn insert(&mut self, player_name: &str, banned_time: usize, sid: usize){

        //Adds name to tree's name storage.
        let name_index = self.name_store.insert(player_name);

        let indexing_result = self._set_child(name_index, sid,
            self.nodes_in_tree, self.root_index, 0);

        //Performs recursion to index the newly inserted data correctly.
        let node_can_be_inserted = indexing_result.0;
        let ending_index = indexing_result.1;
        let needs_rebuild = indexing_result.2;
        
        //If a new node is being inserted, it's "allocated" onto the arena,
        // otherwise the sids are updated and it's all left alone.
        if node_can_be_inserted || self.nodes_in_tree == 0{
            //Allocates new node and updates node count.
            self.data.push(ScapeNode::new(name_index, banned_time, sid));
            self.nodes_in_tree += 1;

            //Calls necessary functions if rebuild of subtree is needed.
            if needs_rebuild{
                let scape_res = self._find_scapegoat(self.visited_nodes.len() - 1, -1, 0);
                let scape_node = scape_res.0;
                let scape_parent: i32;

                //If node has a parent the parent is set appropriately. 
                // If the scapegoat node is root, it has no parent.
                if scape_res.1 != 0{
                    scape_parent = self.visited_nodes[scape_res.1 - 1].0 as i32;
                }else{
                    scape_parent = -1;
                }

                //Uses recursion and stuff to rebuild the desired subtree.
                self._tree_rebuild(scape_parent, scape_node);

            }

        }else{
            //Updates ban time to more recent value if needed.
            if banned_time > self.data[ending_index].time_banned{
                self.data[ending_index].time_banned = banned_time;
            }

            //Updates sids if a new one is found.
            if !self.data[ending_index].sids.contains(&sid){
                self.data[ending_index].sids.push(sid);
            }

        }

        //Resets visited nodes and flattened nodes for later.
        self.visited_nodes.clear();
    }

    //Traverses the tree pre-order style.
    // Generally just for debugging.
    fn _traverse_pre_order(&self, index: i32){
        if index == -1 || self.nodes_in_tree == 0{
            print!("NULL "); 
            return; 
        }

        let i = index as usize;

        print!("{} ", self.name_store.names[self.data[i].player_name_index]);
        self._traverse_pre_order(self.data[i].left);
        self._traverse_pre_order(self.data[i].right);
    }

    //Traverses the tree in-order style.
    // Generally just for debugging.
    fn _traverse_in_order(&self, index: i32){
        if index == -1 || self.nodes_in_tree == 0{ 
            print!("NULL ");
            return; 
        }

        let i = index as usize;

        self._traverse_in_order(self.data[i].left);
        print!("{} ", self.name_store.names[self.data[i].player_name_index]);
        self._traverse_in_order(self.data[i].right);
    }

    //Traverses the tree post-order style.
    // Generally just for debugging.
    fn _traverse_post_order(&self, index: i32){
        if index == -1 || self.nodes_in_tree == 0{  
            print!("NULL ");
            return; 
        }

        let i = index as usize;

        self._traverse_post_order(self.data[i].left);
        self._traverse_post_order(self.data[i].right);
        print!("{} ", self.name_store.names[self.data[i].player_name_index]);
    }

    //Searches tree and if a node is found, the found_node reference is set 
    // to the index of the node found, otherwise, the found_node is left unchanged.
    fn search(&self, search_name: &str, index: i32, found_index: &mut i32){
        if index == -1{
            return;
        }

        let i = index as usize;

        //If node containing name being looked for, 
        // set to that found index and return.
        if search_name == &self.name_store.names[self.data[i].player_name_index]{
            //Makes it so the index is set 
            // to the first node seen with the name.
            if *found_index == -1{
                *found_index = index;
            }
            return;
        }

        //Recurses to left or right child to continue search.
        if search_name < &self.name_store.names[self.data[i].player_name_index]{
            self.search(search_name, self.data[i].left, found_index);
        }else{
            self.search(search_name, self.data[i].right, found_index);
        }

    }

    //Iterates through griefers string read from file, 
    // breaks it up on white-space, 
    // and inserts the information into the scapegoat tree.
    fn parse_griefers_for_scape(&mut self, griefers: String){
        //Splits up input string by whitespace.
        let items: Vec<&str> = griefers.split_whitespace().collect();
        let rows: usize = items.len() / 3;
        
        //Parses needed row information for a griefer and adds it to tree.
        for row in 0..rows{
            let name = items[row * 3];
            let sid: usize = items[(row * 3) + 1].parse().expect("PARSE FAILED");
            let ban_time: usize = items[(row * 3) + 2].parse().expect("PARSE FAILED");
            self.insert(name, ban_time, sid);
        }
    }

    //Iterates through query list and searches tree for names.
    fn search_names(&self, output: &mut BufWriter<std::io::Stdout>, search_names: &Vec<&str>){
        let size = search_names.len();
        let mut search_index = -1;
        
        //Queries each name and displays the needed information.
        for i in 0..size{
            self.search(search_names[i], self.root_index as i32, &mut search_index); 

            if search_index != -1{
                let ban_count = self.data[search_index as usize].sids.len();
                let ban_time = self.data[search_index as usize].time_banned;
                
                //Displays ban info for player.
                writeln!(output, "{} was banned from {} servers. most recently on: {}",
                    search_names[i], ban_count, ban_time)
                    .expect("WRITING FAILED");
            }else{
                writeln!(output, "{} is not currently banned from any servers.", search_names[i])
                    .expect("WRITING FAILED.");
            }


            search_index = -1;
        }
    } 
}

struct NodeAVL{
    player_name_index: usize,
    sids: Vec<usize>,
    time_banned: usize,
    children: [isize; 2],
    balance: i8
}

impl NodeAVL{
    fn new(name_index: usize, banned_time: usize, sid: usize) -> Self{
        let mut new_node = NodeAVL{
            player_name_index: name_index, 
            sids: Vec::new(),
            time_banned: banned_time,
            children: [-1, -1],
            balance: 0
        };
        new_node.sids.push(sid);
        new_node
    }
}

struct TreeAVL{
    nodes_in_tree: usize,
    root_index: usize,
    data: Vec<NodeAVL>,
    name_store: NameContainer
}

impl TreeAVL{
    fn new(mut capacity: usize) -> Self{
        if capacity == 0{capacity = 64;}

        TreeAVL{
            nodes_in_tree: 0,
            root_index: 0,
            data: Vec::with_capacity(capacity),
            name_store: NameContainer::new(capacity)
        }
    }

    //Rotates three nodes in a basic direction left or right.
    fn rotate(&mut self, old_root: usize, direction: usize, adjust_bal: bool) -> isize{
        //Doesn't rotate if configuration is invalid.
        if self.data[old_root].children[1 - direction] == -1 {return old_root as isize;}

        //Establishes former child node as new root for rotated subtree.
        let child = self.data[old_root].children[1 - direction] as usize;
        let new_root = child; 

        //Replaces desired child of old root 
        // with opposite direction child of desired child.
        self.data[old_root].children[1 - direction] = self.data[child].children[direction];
        self.data[new_root].children[direction] = old_root as isize;

        //Adjusts balance if needed.
        if adjust_bal{
            self.data[new_root].balance = 0;
            self.data[old_root].balance = 0;
        }

        return new_root as isize;

    }

    //Performs multi-rotation case.
    fn multi_rotate(&mut self, old_root: usize, type_of_rot: usize) -> isize{
        //Rotates child of old root to make it a straight sub-tree 
        // to then perform final rotation.
        let old_child = self.data[old_root].children[type_of_rot];
        let new_child = self.rotate(old_child as usize, type_of_rot, false);
        self.data[old_root].children[type_of_rot] = new_child;

        //Performs whole sub-tree rotation to balance it.
        let new_root = self.rotate(old_root, 1 - type_of_rot, false);

        let left_child = self.data[new_root as usize].children[0];
        let right_child = self.data[new_root as usize].children[1];

        //Sets up appropriate balance factors of rotated subtree.
        
        //If new root is balanced, the children are balanced.
        if self.data[new_root as usize].balance == 0{
            self.data[left_child as usize].balance = 0;
            self.data[right_child as usize].balance = 0;
        }else{
            //If the new root is skewed to the right, 
            // the left child is skewed to the left.
            // Otherwise, the right child is skewed right.
            if self.data[new_root as usize].balance > 0{
                self.data[left_child as usize].balance = -1;
                self.data[right_child as usize].balance = 0;
            }else{
                self.data[left_child as usize].balance = 0;
                self.data[right_child as usize].balance = 1;
            }
            //Balance set to 0.
            self.data[new_root as usize].balance = 0;
        }

        new_root
    }

    //Inserts node into AVL tree.
    fn insert(&mut self, player_name: &str, banned_time: usize, sid: usize, 
            parent_index: isize, curr_index: usize){

        //Empty tree case. Inserts node and returns.
        if self.nodes_in_tree == 0{
            self.data.push(NodeAVL::new(
                self.name_store.insert(player_name), banned_time, sid));
            self.nodes_in_tree += 1;
            return;
        }

        //If griefer already in tree, updates node information for griefer.
        if &self.name_store.names[self.data[curr_index].player_name_index] == player_name{
            //Updates banned time if more recent ban time was found.
            if banned_time > self.data[curr_index].time_banned{
                self.data[curr_index].time_banned = banned_time;
            }
            
            //If unique sid found, push it.
            if !self.data[curr_index].sids.contains(&sid){
                self.data[curr_index].sids.push(sid);
            }
            
            return;
        }

        //Determines which child to go to next.
        let left_right_index: usize = (player_name >
            &self.name_store.names[self.data[curr_index].player_name_index]) as usize; 
        let plus_minus_arr = [-1, 1];

        //Inserts node into tree if empty child found. 
        // Otherwise traverses to next child.
        if self.data[curr_index].children[left_right_index] == -1{
            //Allocates new node for tree.
            self.data.push(NodeAVL::new(
                self.name_store.insert(player_name), banned_time, sid));
            
            //Child now references new node.
            self.data[curr_index].children[left_right_index] = self.nodes_in_tree as isize;

            self.nodes_in_tree += 1;

            //Alters balance of current node based on new insertion.
            self.data[curr_index].balance += plus_minus_arr[left_right_index];

        }else{
            let child_index = self.data[curr_index].children[left_right_index] as usize;

            let child_balance_before = self.data[child_index].balance;

            //Traverses to next child.
            self.insert(player_name, banned_time, sid, curr_index as isize,
                self.data[curr_index].children[left_right_index] as usize);

            let child_balance_after = self.data[child_index].balance;

            if child_balance_before == 0 && child_balance_after != 0 {
                self.data[curr_index].balance += plus_minus_arr[left_right_index];
            }


        }

        //Two balances acquired to make sure 
        // balance of sub-tree is being preserved.
        let curr_bal: i8 = self.data[curr_index].balance;
        let child_bal: i8 = self.data[ 
            self.data[curr_index].children[left_right_index] as usize
        ].balance;
        
        let needs_right_rot = curr_bal < -1 && child_bal <= -1;
        let needs_left_rot = curr_bal > 1 && child_bal >= 1;
        let needs_left_right_rot = curr_bal < -1 && child_bal >= 1;
        let needs_right_left_rot = curr_bal > 1 && child_bal <= -1;

        let old_root_is_right_child_of_parent: bool;
        if parent_index != -1{
            old_root_is_right_child_of_parent = 
                self.data[parent_index as usize].children[1] == (curr_index as isize);
        }else{
            old_root_is_right_child_of_parent = false;
        }

        //Rotation cases.
        if needs_right_rot{
            let new_root = self.rotate(curr_index, 1, true);
            self.update_proper_parent(parent_index, old_root_is_right_child_of_parent, new_root);
        }
        if needs_left_rot{
            let new_root = self.rotate(curr_index, 0, true);
            self.update_proper_parent(parent_index, old_root_is_right_child_of_parent, new_root);
        }
        if needs_left_right_rot{
            let new_root = self.multi_rotate(curr_index, 0);
            self.update_proper_parent(parent_index, old_root_is_right_child_of_parent, new_root);
        }
        if needs_right_left_rot{
            let new_root = self.multi_rotate(curr_index, 1);
            self.update_proper_parent(parent_index, old_root_is_right_child_of_parent, new_root);
        }

    }

    //Replaces the correct child of the parent of the old root.
    fn update_proper_parent(&mut self, parent_index: isize, 
            old_root_is_right_child_of_parent: bool, new_root: isize){
        //Replaces tree's root if whole tree is being rotated.
        //Otherwise replaces old root as child of rest of tree.
        if parent_index == -1{
            self.root_index = new_root as usize;
        }else{
            self.data[parent_index as usize]
                .children[old_root_is_right_child_of_parent as usize] = new_root;
        }
    }

    //Searches tree and if a node is found, the found_node reference is set 
    // to the index of the node found, otherwise, the found_node is left unchanged.
    fn search(&self, search_name: &str, index: isize) -> isize{
        if index == -1{
            return index;
        }

        let i = index as usize;

        //If node containing name being looked for, 
        // set to that found index and return.
        if search_name == &self.name_store.names[self.data[i].player_name_index]{
            return index;
        }

        //Recurses to left or right child to continue search.
        let left_right_index = (search_name > 
            &self.name_store.names[self.data[i].player_name_index]) as usize;

        return self.search(search_name, self.data[i].children[left_right_index]);

    }

    //Iterates through griefers string read from file, 
    // breaks it up on white-space, 
    // and inserts the information into the AVL tree.
    fn parse_griefers_and_insert(&mut self, griefers: String){
        //Splits up input string by whitespace.
        let items: Vec<&str> = griefers.split_whitespace().collect();
        let rows: usize = items.len() / 3;
        
        //Parses needed row information for a griefer and adds it to tree.
        for row in 0..rows{
            let name = items[row * 3];
            let sid: usize = items[(row * 3) + 1].parse().expect("PARSE FAILED");
            let ban_time: usize = items[(row * 3) + 2].parse().expect("PARSE FAILED");
            self.insert(name, ban_time, sid, -1, self.root_index);
        }
    }

    //Iterates through query list and searches tree for names.
    fn search_names(&self, output: &mut BufWriter<std::io::Stdout>, search_names: &Vec<&str>){
        let size = search_names.len();
        
        //Queries each name and displays the needed information.
        for i in 0..size{
            let search_index = self.search(search_names[i], self.root_index as isize); 

            if search_index != -1{
                let ban_count = self.data[search_index as usize].sids.len();
                let ban_time = self.data[search_index as usize].time_banned;
                
                //Displays ban info for player.
                writeln!(output, "{} was banned from {} servers. most recently on: {}",
                    search_names[i], ban_count, ban_time)
                    .expect("WRITING FAILED");
            }else{
                writeln!(output, "{} is not currently banned from any servers.", search_names[i])
                    .expect("WRITING FAILED.");
            }
        }
    } 
}

fn main() {
    let start = Instant::now();
    
    //Gets argv and argc
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    
    //Checks to make sure some kind of command line argument is given.
    if argc < 3{panic!("Insufficient args given!!!!!!!!");}

    //Checks to see if command line argument given for tree is valid.
    let is_scape = argv[1] == "scapegoat";
    let is_avl = argv[1] == "avl";

    //If first command line arg isn't one of the two valid ones, program stops.
    if !is_scape && !is_avl {panic!("Invalid tree argument given!");}

    //Reads in requested players.
    let mut input_players = String::new();
    std::io::stdin()
        .read_to_string(&mut input_players)
        .expect("FAILED TO READ FILE");
    
    //Establishes buffer used to write sorted output.
    let mut out_buff: BufWriter<std::io::Stdout> = BufWriter::new(std::io::stdout());
    
    //Opens file based on name given in command line.
    let path = Path::new(&argv[2]);
    let path_name = path.display();
    let mut data_file = match File::open(Path::new(&argv[2])){
        Err(reason) => panic!("File {} couldn't be opened reason: {}", path_name, reason),
        Ok(file) => file,
    };

    //Reads in griefer data from file.
    let mut griefer_file_string = String::new();
    data_file.read_to_string(&mut griefer_file_string).expect("READING FAILED");
    
    //Gathers player names into list of strings for future inquiry.
    let player_names: Vec<&str> = input_players.split_whitespace().collect();

    //Runs scapegoat tree if the argument is as such.
    if is_scape{
        let mut tree = ScapeTree::new(200000);
        tree.parse_griefers_for_scape(griefer_file_string);
        tree.search_names(&mut out_buff, &player_names);
    }

    //Runs if argument is avl.
    else if is_avl{
        let mut avl = TreeAVL::new(200000);
        avl.parse_griefers_and_insert(griefer_file_string);
        avl.search_names(&mut out_buff, &player_names);
    }


    //Finds time taken for program to run.
    let duration: u128 = start
        .elapsed()
        .as_micros();

    //Spits out final output.
    writeln!(out_buff, "total time in microseconds: {}", duration).expect("WRITING FAILED");
}
