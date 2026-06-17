fn main() {
    println!("Hello, world!");

    let raiz = raiz_quadrada(-1.0);
    match raiz {
        Ok(r) => println!("Raiz: {}", r),
        Err(e) => println!("Erro: {}", e),
    }
}

fn raiz_quadrada(num: f64) -> Result<f64, String> {
    if num < 0.0 {
        Err(String::from("Número negativo não tem raiz quadrada real"))
    } else {
        Ok(num.sqrt())
    }
}
