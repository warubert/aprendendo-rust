mod animais;

fn main() {
    println!("Hello, world!");
    animais::correr();
    animais::mamiferos::amamentar();
    animais::mamiferos::gato::miar();
}
