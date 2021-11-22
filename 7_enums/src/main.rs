// Enums

#[derive(Debug)]
struct ColorRGB {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug)]
struct ColorGreyscale {
    grey: u32
}

#[derive(Debug)]
enum MyColor {
    RGB(ColorRGB),
    Grey(ColorGreyscale)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(MyColor)
}

fn message_run() {
    let m_quit: Message = Message::Quit;
    let m_move: Message = Message::Move {x: 32, y: 32};
    let m_write: Message = Message::Write(String::from("::1"));
    
    let color_rgb: ColorRGB = ColorRGB {red: 255, green: 255, blue:255};
    let color: MyColor = MyColor::RGB(color_rgb);
    let m_change_color_rgb: Message = Message::ChangeColor(color);

    let m_change_color_grey: Message = Message::ChangeColor(MyColor::Grey(ColorGreyscale{grey:32}));

    println!("{:?}", m_quit);
    println!("{:?}", m_move);
    println!("{:?}", m_write);
    println!("{:?}", m_change_color_rgb);
    println!("{:?}", m_change_color_grey);
}

fn main() {
    message_run();
}
