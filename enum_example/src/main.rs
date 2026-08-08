#[derive(Debug)]
enum WebEvent {
    PageLoad,
    Click { x: i64, y: i64 },
    KeyPress(char),
    Paste(String),
}

fn main() {
    println!("---- Creating different WebEvent ----");
    let load = WebEvent::PageLoad;
    let click = WebEvent::Click { x: 20, y: 80 };
    let key = WebEvent::KeyPress('x');
    let paste = WebEvent::Paste("my text".to_string());

    println!("{:?}", load);
    println!("{:?}", click);
    println!("{:?}", key);
    println!("{:?}", paste);
}
