pub fn solve(largest_entry: usize) -> Vec<usize> {
    let mut best_array = vec![];
    best_array.reserve(largest_entry);
    let mut current_array = vec![0];
    current_array.reserve(largest_entry);
    let mut sums = vec![false; largest_entry * largest_entry + 2];
    loop {
        while !current_array.is_empty() && current_array[current_array.len() - 1] == largest_entry {
            current_array.pop();
        }
        if current_array.is_empty() {
            break;
        }
        let last_index = current_array.len() - 1;
        current_array[last_index] += 1;
        let mut failed = false;
        'outer: for sub_start in 0..current_array.len() {
            let mut sum = 0;
            for sub_end in sub_start..current_array.len() {
                sum += current_array[sub_end];
                if sums[sum] {
                    failed = true;
                    break 'outer;
                }
                sums[sum] = true;
            }
        }
        if !failed {
            if current_array.len() > best_array.len() {
                best_array = current_array.clone();
            }
            current_array.push(0);
        }
        for i in 0..sums.len() {
            sums[i] = false;
        }
    }
    best_array
}

fn main() {
    for largest_entry in 1..15 {
        let result = solve(largest_entry);
        println!("{}: {} - {:?}", largest_entry, result.len(), result);
    }
}
