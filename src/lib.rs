pub fn solve(largest_entry: usize) -> Vec<usize> {
    let mut best_array = vec![];
    best_array.reserve(largest_entry);
    let mut current_array = vec![0];
    current_array.reserve(largest_entry);
    // In this version, sums ignores the sums involving the final element
    let mut sums = vec![false; largest_entry * largest_entry + 2];
    loop {
        let mut recalculate = false;
        while !current_array.is_empty() && current_array[current_array.len() - 1] == largest_entry {
            current_array.pop();
            recalculate = true;
        }
        if current_array.is_empty() {
            break;
        }
        let last_index = current_array.len() - 1;
        current_array[last_index] += 1;
        let mut failed = false;
        if recalculate {
            for sum in &mut sums {
                *sum = false;
            }
            'outer: for sub_start in 0..current_array.len() {
                let mut sum = 0;
                for sub_end in sub_start..current_array.len() - 1 {
                    sum += current_array[sub_end];
                    if sums[sum] {
                        failed = true;
                        break 'outer;
                    }
                    sums[sum] = true;
                }
            }
        }
        let mut sum = 0;
        for add_index in (0..current_array.len()).rev() {
            sum += current_array[add_index];
            if sums[sum] {
                failed = true;
                break;
            }
        }
        if !failed {
            if current_array.len() > best_array.len() {
                best_array = current_array.clone();
            }
            let mut sum = 0;
            for add_index in (0..current_array.len()).rev() {
                sum += current_array[add_index];
                sums[sum] = true;
            }
            current_array.push(0);
        }
    }
    best_array
}

fn main() {
    let max_largest_entry: usize = std::env::args()
        .nth(1)
        .expect("Provide the maximum largest entry as a command line parameter")
        .parse()
        .expect("Parameter should be a positive integer");
    for largest_entry in 1..=max_largest_entry {
        let result = solve(largest_entry);
        println!("{}: {} - {:?}", largest_entry, result.len(), result);
    }
}
