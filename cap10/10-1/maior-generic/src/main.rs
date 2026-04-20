struct Ponto<T, U> {
    x: T,
    y: U,
}

struct Ponto<T> {
    x: T,
    y: T,
}

impl<T> Ponto<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let ambos_inteiros = Ponto { x: 5, y: 10 };
    let ambos_floats = Ponto { x: 1.0, y: 4.0 };
    let inteiro_e_float = Ponto { x: 5, y: 4.0 };

    let p = Ponto { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}