use std::fs::File;
use std::path::Path;

fn main() {
    // open file for read
    let fname = "lists.txt";
    let path = Path::new(fname);
    let mut file = File::open(path)
    // form two lists, left and right
    // sort left and right lists in ascending order
    // extract location ids in order, calculate distance
    // at to total sum
}

