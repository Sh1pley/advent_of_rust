mod utilities;
use crate::utilities::file_parse::get_lines;

fn main() {
    if let Ok(lines) = get_lines("./data/input.txt") {
        // get total count of lines?
        //println!("number of lines: {:?}", size);
        let mut list_a: Vec<i32> = Vec::new();
        let mut list_b: Vec<i32> = Vec::new();

        let mut diff_list: Vec<i32> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                // break each line pair into part a / part b
                let mut parts = ip.split_ascii_whitespace(); 
                let part_a: i32 = parts.next().unwrap().parse().expect("Part A should be here.");
                let part_b: i32 = parts.next().unwrap().parse().expect("Part b should be here.");
                // assign each object pair to some sort of list to compare
                list_a.push(part_a);
                list_b.push(part_b);
            }
        }
        // iterate through each list comparing [i] for [i] building new list with difference
        // keep track of an index on our own? probably not ideal
        let mut i = 0;
        // I guess rust offers a mutational sort function. Nice!
        list_a.sort();
        list_b.sort();
        for item in list_a{
            // get difference between [i] of each list
            let nitem = list_b[i];
            let diff: i32 = item.abs_diff(nitem) as i32;
            diff_list.push(diff);
            i += 1
        }
        let sum: i32 = diff_list.iter().sum();
        println!("{:?}", sum);
    }
}

