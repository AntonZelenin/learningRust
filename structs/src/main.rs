fn main() {
    println!("Hello, world!");

    let mut user = User {
        username: String::from("AZelenin"),
        active: true,
        email: String::from("ntnzelenin@gmail.com"),
        sign_in_count: 1
    };

    user.email = String::from("mail@email.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        // here initialization was simplified in order to avoid code duplicating
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
