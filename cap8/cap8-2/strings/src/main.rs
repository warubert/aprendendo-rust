fn main() {
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // o método também funciona em literais diretamente
    let s = "initial contents".to_string();

    // let hello = "السلام عليكم";
    // let hello = "Dobrý den";
    // let hello = "Hello";
    // let hello = "שָׁלוֹם";
    // let hello = "नमस्ते";
    // let hello = "こんにちは";
    // let hello = "안녕하세요";
    // let hello = "你好";
    // let hello = "Olá";
    // let hello = "Здравствуйте";
    // let hello = "Hola";

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note que s1 foi movido aqui e não pode ser mais usado

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
