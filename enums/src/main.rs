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

fn main() {
    let mut player1 = PlayerState::Alive { hp: 20 };
    dbg!(&player1);

    player1.kill();
    dbg!(&player1);

    match player1 {
        PlayerState::Alive { hp } => println!("Alive, hp: {}", player1.hp()),
        PlayerState::Dead => println!("Dead, hp: {}", player1.hp()),
    }
}
