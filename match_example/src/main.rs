#[derive(Debug)]
enum WebEvent {
    PageLoad,
    Click { x: i64, y: i64 },
    KeyPress(char),
    Paste(String),
}

fn handle_event(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::Click { x, y } => println!("Clicked at ({}, {})", x, y),
        WebEvent::KeyPress(c) => println!("Key pressed: {}", c),
        WebEvent::Paste(s) => println!("Pasted: {}", s),
    }
}
fn main() {
    println!("---- Creating different WebEvent ----");
    let load = WebEvent::PageLoad;
    let click = WebEvent::Click { x: 20, y: 80 };
    let key = WebEvent::KeyPress('x');
    let paste = WebEvent::Paste("my text".to_string());

    handle_event(&load);
    handle_event(&click);
    handle_event(&key);
    handle_event(&paste);
}
