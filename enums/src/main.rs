#[derive(Debug)]
enum PlayerState {
    Alive { hp: u32 },
    Dead,
}

impl PlayerState {
    fn hp(&self) -> u32 {
        match *self {
            PlayerState::Alive { hp } => hp,
            PlayerState::Dead => 0,
        }
    }

    fn kill(&mut self) -> () {
        *self = PlayerState::Dead;
    }
}

#[allow(unused)]  // suppress warnings
fn main() {
    // Enums
    let mut player1 = PlayerState::Alive { hp: 20 };
    dbg!(&player1);

    player1.kill();
    dbg!(&player1);

    match player1 {
        PlayerState::Alive { hp: _ } => println!("Alive, hp: {}", player1.hp()),  // hp: _ means we don't care about the value
        PlayerState::Dead => println!("Dead, hp: {}", player1.hp()),
    }

    // Option
    let five: i32 = 5;
    let some_5 = Some(5);
    let no_num: Option<i32> = None;

    // let sum = five + no_num;  // bad (no implementation for `i32 + Option<i32>`)
    // use Option<T> with match.
    /*
    methods:
    - is_some() -> bool
    - is_none() -> bool
    - unwrap() -> T (panics if the value is None) 
    - unwrap_or(T) -> T (returns the value or a default)
    - expect(&str) -> T (panics with a custom message)
    - as_ref() -> Option<&T> (converts from &Option<T> to Option<&T>)
    - as_mut() -> Option<&mut T> (converts from &mut Option<T> to Option<&mut T>)
    - Others: https://doc.rust-lang.org/std/option/enum.Option.html
     */

}
