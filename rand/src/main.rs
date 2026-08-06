use rand::Rng;

fn main() {
    println!("Número aleatório entre 1 e 100!");

    let mut rng = rand::thread_rng();
    let numero_aleatorio = rng.gen_range(1..=100);
    println!("Número aleatório: {}", numero_aleatorio);
}
