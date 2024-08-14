fn main() {
    let s = "Hello Rust this is a String Slice (&str).";
    let n: i32 = 2;
    let word = nth_word(&s, n);

    println!("{s}");
    println!("Word #{n} is \"{word}\"");
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

    &s[..]
}
