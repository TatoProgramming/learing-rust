
// Each enum can unique "overloaded constructors" - not sure of the name.
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
//
// let home = IpAddr::V4(127, 0, 0, 1);
//
// let loopback = IpAddr::V6(String::from("::1"));

// This is the same as the above
// enum IpAddrKind{
//     V4,
//     V6,
// }
//
// struct IpAddr{
//     kind: IpAddrKind,
//     address: String,
// }
//
// let home = IpAddr{
//     kind: IpAddrKind::V4,
//     address: Strings::from("127.0.0.1"),
// };
//
// let loopback = IpAddr{
//     kind: IpAddrKind::V6,
//     address: Strings::from("::1"),
// };

enum IpAddr{
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V4(String::from("::1"));
}
