fn main() {
    // struct instantiation:
    let mut alice = User {
        email: String::from("aIice@example.com"),
        username: String::from("Alice"),
        active: true,
        sign_in_count: 1,
    };
    alice.email = String::from("alice@example.com");

    let bob = build_user(String::from("bob@example.com"), String::from("Bob"));
    println!("bob email: {}", bob.email);

    let alice_alt = User {
        email: String::from("alice2@example.com"),
        username: alice.username,
        active: alice.active,
        sign_in_count: alice.sign_in_count,
    };
    println!("alice_alt email: {}", alice_alt.email);

    /* struct update syntax:
    // this copies all fields from alice to alice_alt, except for email
    let alice_alt = User {
        email: String::from("alice2@example.com");
        ..alice
        };
    */

    // data was moved to alice_alt, so alice is no longer valid (particularly, username was moved)
    // println!("alice: {}", alice.username);  // error: value borrowed here after move

    // tuple struct:
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let x = origin.0;
    let y = origin.1;
    let z = origin.2;

    // let (x, y, z) = origin;  // destructuring

    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}, {}, {})", x, y, z);

    // unit-like struct:
    let _unit_like = UnitLikeStruct;  // _ to avoid warning about unused variable

    // burrowed fields:
    let email = &alice.email;

    // other fields won't lose ownership:
    alice.sign_in_count += 1; 

    // but the burrowed field loses ownership, and so does the struct itself:
    // println!("alice email: {}", alice.email);  // error: value borrowed here after move

    println!("alice email: {}", email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

/* field init shorthand:
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

*/
// tuple struct:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// unit-like struct:
struct UnitLikeStruct;

// NOTE: Struct fields can be references, but they need to have a lifetime specifier. We do that later ig.
