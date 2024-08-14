fn main() {
    // String Slices
    let s = "Hello Rust this is a String Slice (&str).";
    let n: i32 = 2;

    let word = nth_word(&s, n);
    let word2 = nth_word_alt(s, n);

    println!("{s}");

    println!("(nth_word) Word #{n} is \"{word}\"");
    println!("(nth_word_alt) Word #{n} is \"{word2}\"");

    // Array Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[2..4];

    assert_eq!(slice, &[3, 4]);
}

fn nth_word(s: &str, n: i32) -> &str {
    let bytes = s.as_bytes();
    let mut word_count: i32 = 0;
    let mut first_index: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if word_count < n-1 {
                word_count += 1;
                first_index = i + 1;
                continue
            }

            return &s[first_index..i]
        }

    }

    &s[first_index..]
}


// Direcyly iterarte over the characters
fn nth_word_alt(s: &str, n: i32) -> &str {
    let mut word_count = 0;
    let mut first_index = 0;

    for (i, c) in s.char_indices() {  // Iterate over characters and their indices
        if c == ' ' {
            if word_count < n - 1 {
                word_count += 1;
                first_index = i + 1;
                continue;
            }
            return &s[first_index..i];
        }
    }

    &s[first_index..] // Return the last word if the loop completes without returning
}
