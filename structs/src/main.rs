struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("david-why"),
        email: String::from("david_why@outlook.com"),
        sign_in_count: 1,
    };

    let black = Color(0, 0, 0);

    let subject = AlwaysEqual;

    let user2 = User {
        active: true,
        ..user1
    };

    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("{}", i);
    }
}

fn make_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
