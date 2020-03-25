pub fn run() {
    hi();
    let result = is_even(50);
    println!("{}", result);

    let mut array: [i32; 6] = [6, 5, 4, 3, 2, 1];

    sort(&mut array);
    println!("Sorted Array: {:?}", array)
}

// Regular old function
fn hi() {
    println!("Hello World!");
}

// Function with a return
fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}

// Simple Sorting Function
fn sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[j] < nums[i] {
                nums.swap(i, j);
            }
        }
    }
}
