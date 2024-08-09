fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // break can return a value
        }
    };

    println!("The result is {result}");

    // loops can have 'labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loops
    let mut counter = 3;

    while counter != 0 {
        println!("{}!", counter);

        counter -= 1;
    }
    
    // looping over an array
    let a = [20, 30, 40, 50, 60];
    let mut index = 0;
    let max_index = a.len();

    while index < max_index {
        println!("Element: {}", a[index]);
        index += 1;
    }

    // for altermatives
    for number in (1..4).rev() {
        println!("{number}!");
    }

    for element in a {
        println!("Element: {}", element);
    }
}