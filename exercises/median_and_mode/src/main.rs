use std::collections::HashMap;

fn main() {
    let list = vec![8,8,1,6,2,5,3,1,5,4,6,3,2,1,7,4];
    
    let median = get_median(&list);

    let mode = get_mode(&list);

    println!("For the list {:?}, the median is {} and the mode is {}", list, median, mode);
}

fn get_median(list: &Vec<i32>) -> f64 {
    let mut sorted = list.clone();
    sorted.sort();

    let len = sorted.len();
    let mid = len / 2;
    match len % 2 {
        0 => (sorted[mid] as f64 + sorted[mid-1] as f64) / 2.0,
        _ => sorted[mid] as f64,
    }
}

fn get_mode(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    
    let mut mode = 0;
    let mut count = 0;
    for (key, value) in map {
        if value > count {
            mode = key;
            count = value;
        }
    }
    mode
}
