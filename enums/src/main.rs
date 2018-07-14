fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let home2 = IpAddrKind2::V4(String::from("127.0.0.1"));
    println!("{:?}", home2);

    let home3 = IpAddrKind3::V4(127, 0, 0, 1);
    println!("{:?}", home3);

    let m = Message::Write(String::from("hello"));
    m.call();


    let some_number = Some(5);
    let some_string = Some("hello Some");

    let mut absent_number: Option<i32> = None;
    absent_number = Some(5);
}

enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum IpAddrKind4 {
    V4(IpAddr),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn route(ip_type: IpAddrKind) { }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}
