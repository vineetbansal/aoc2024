use std::fs;


fn similarity1(a: Vec<u32>, b: Vec<u32>) -> u32 {
    let mut similarity: u32 = 0;
    for i in 0..a.len() {
        similarity += u32::abs_diff(a[i], b[i]);
    }
    similarity
}


fn similarity2(a: Vec<u32>, b: Vec<u32>, b_is_sorted: bool) -> u32 {
    let mut similarity: u32 = 0;
    for i in 0..a.len() {
        let x = a[i];
        if b_is_sorted {
            // binary search
            let low = b.partition_point(|y| y < &x);
            let high = b.partition_point(|y| y <= &x);
            if high > low {
                similarity += ((high - low) as u32) * x;
            }
        } else {
            // linear search
            for j in 0..b.len() {
                if b[j] == x {
                    similarity += x;
                }
            }
        }
    }
    similarity
}


fn main() {
    let document = fs::read_to_string("input1_1.txt").unwrap();
    let mut arr1:Vec<u32> = Vec::new();
    let mut arr2:Vec<u32> = Vec::new();
    for line in document.lines() {
        let splits: Vec<u32> = line.split("   ").map(|s| s.parse::<u32>().unwrap()).collect();
        arr1.push(splits[0]);
        arr2.push(splits[1]);
    }
    arr1.sort();
    arr2.sort();
    assert_eq!(arr1.len(), arr2.len());

    print!("{:?}", similarity2(arr1, arr2, true));
}
