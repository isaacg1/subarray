pub fn solve(largest_entry: usize) -> Vec<usize> {
    for target_length in 1..=largest_entry {
        let result = solve_inner(largest_entry, target_length);
        if result.len() < target_length || result.len() == largest_entry {
            return result
        }
    }
    unreachable!()
}

fn solve_inner(largest_entry: usize, target_length: usize) -> Vec<usize> {
    let mut best_array = vec![];
    best_array.reserve(largest_entry);
    let mut current_array = vec![0];
    current_array.reserve(largest_entry);
    // In this version, sums ignores the sums involving the final element
    // Ignores bigger than half
    let elems_needed = largest_entry * (largest_entry + 1) / 2 + 2;
    let mut sums = vec![false; elems_needed + 4 - elems_needed % 4];
    'outermost: loop {
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
        if recalculate {
            for sum in sums.iter_mut() {
                *sum = false;
            }
            for sub_start in 0..current_array.len() {
                let mut sum = 0;
                for sub_end in sub_start..current_array.len() - 1 {
                    sum += current_array[sub_end];
                    if sums[sum] {
                        continue 'outermost
                    }
                    sums[sum] = true;
                }
            }
        }
        let mut sum = 0;
        for add_index in (0..current_array.len()).rev() {
            sum += current_array[add_index];
            if sums[sum] {
                continue 'outermost
            }
        }
        if current_array.len() > best_array.len() {
            best_array.clone_from(&current_array);
            if best_array.len() == target_length {
                return best_array
            }
        }
        let mut sum = 0;
        for entry in current_array.iter().rev() {
            sum += entry;
            sums[sum] = true;
        }
        let midpoint = (target_length+1)/2;
        if current_array.len() <= midpoint {
            current_array.push(0);
        } else {
            current_array.push(*current_array.iter().take(midpoint).min().unwrap())
        }
    }
    best_array
}

#[allow(dead_code)]
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
