fn cria_somador(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn soma(x: i32, y: i32) -> impl Fn(i32) -> i32 {
    let r = x + y;
    move |multiplicador| r*multiplicador
}

fn main() {
    let somador = cria_somador(5);
    println!("{}", somador(3)); // Saída: 8

    let resultado_soma = soma(2, 3);
    let resultado_final = resultado_soma(4);
    println!("{}", resultado_final); // Saída: 20

    let salario = 5000.0;

    fn aplicar_desconto(salario: f64, descontos: Vec<fn(f64) -> f64>) -> f64 {
        descontos.iter().fold(salario, |acc, desconto| desconto(acc))
    }

    let salario_liquido = aplicar_desconto(salario, vec![
        |s| s * 0.9, // Desconto de 10%
        |s| s - 0.95, // Desconto fixo de 0.95
        |s| s * 0.97, // Desconto de 3%
    ]);
    println!("Salário líquido: {:.2}", salario_liquido);
}
