fn main() {
    print_new_user();
    struct_tuple_show();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn print_new_user() {
    let mut user = User {
        email: String::from("test@domain.com"),
        username: String::from("tester"),
        sign_in_count: 0,
        active: true,
    };

    println!("new_user: {} / {}", user.username, user.email);
    user.email = String::from("test2@domain.com");
    println!("new_user: {} / {}", user.username, user.email);

    let user = new_user("another@dot.com".to_string(), "John".to_string());
    println!("new_user: {} / {} / {} is {}", user.username, user.email, user.sign_in_count, user.active);

    copy_update_user(&user);
}

fn new_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 5,
        active: true,
    }
}

fn copy_update_user(src: &User) {
    let user = User {
        email: String::from("copied@domain.com"),
        username: String::from("copied_guy"),
        ..(*src)
    };

    println!("copy_update_user: {} / {}/ {}", user.username, user.email, user.sign_in_count);
}

struct Color(u32, u32, u32);

fn struct_tuple_show() {
    let black = Color(0, 0, 0);
    println!("struct_tuple_show: {}{}{}", black.0, black.1, black.2);
}
