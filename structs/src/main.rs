struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut alice = User {
        email: String::from("aIice@example.com"),
        username: String::from("Alice"),
        active: true,
        sign_in_count: 1,
    };

    alice.email = String::from("alice@example.com");

    let bob = build_user(
        String::from("bob@example.com"),
        String::from("Bob"),
    );

    let alice_alt = User {
        email: String::from("alice2@example.com"),
        username: alice.username,
        active: alice.active,
        sign_in_count: alice.sign_in_count,
    };

    /* struct update syntax:
    let alice_alt = User {
        email: String::from("alice2@example.com");
        ..alice
    };
     */
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
