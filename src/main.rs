// Question 1: Implement a function that checks whether a given string is a palindrome or not.
fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

// Question 2: Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
fn first_occurrence_index(arr: &[i32], num: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == num {
            return Some(index);
        }
    }
    None
}

// Question 3: Given a string of words, implement a function that returns the shortest word in the string.
fn shortest_word(s: &str) -> &str {
    s.split_whitespace().min_by_key(|word| word.len()).unwrap_or_default()
}

// Question 4: Implement a function that checks whether a given number is prime or not.
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Question 5: Given a sorted array of integers, implement a function that returns the median of the array.
fn median(arr: &[i32]) -> f64 {
    let mid = arr.len() / 2;
    if arr.len() % 2 == 0 {
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[mid] as f64
    }
}

// Question 6: Implement a function that finds the longest common prefix of a given set of strings.
fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = strings[0].to_string();
    for s in strings.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}

// Question 7: Implement a function that returns the kth smallest element in a given array.
fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted = arr.to_vec();
    sorted.sort();
    sorted.get(k - 1).copied()
}

fn main() {
    // Question 1: Palindrome check
    {
        let s = "radar";
        if is_palindrome(s) {
            println!("'{}' is a palindrome.", s);
        } else {
            println!("'{}' is not a palindrome.", s);
        }
    }

    // Question 2: First occurrence index
    {
        let arr = [1, 3, 5, 7, 9];
        let num = 5;
        
        if let Some(index) = first_occurrence_index(&arr, num) {
            println!("First occurrence of {} is at index {}", num, index);
        } else {
            println!("Number {} not found in the array", num);
        }
    }

    // Question 3: Shortest word
    {
        let sentence = "The quick brown fox jumps over the lazy dog";
        let shortest = shortest_word(sentence);
        
        println!("Shortest word: {}", shortest);
    }

    // Question 4: Prime check
    {
        let num = 17;
        
        if is_prime(num) {
            println!("{} is a prime number", num);
        } else {
            println!("{} is not a prime number", num);
        }
    }

    // Question 5: Median of array
    {
        let arr = [1, 3, 5, 7, 9];
        let m = median(&arr);
        
        println!("Median: {}", m);
    }

    // Question 6: Longest common prefix
    {
        let strings = ["flower", "flow", "flight"];
        let lcp = longest_common_prefix(&strings);
        
        println!("Longest Common Prefix: {}", lcp);
    }

    // Question 7: Kth smallest element
    {
        let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5];
        let k = 3;
        
        if let Some(smallest) = kth_smallest_element(&arr, k) {
            println!("{}th smallest element: {}", k, smallest);
        } else {
            println!("Invalid input or index out of range");
        }
    }
}
