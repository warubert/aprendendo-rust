const TIPO_DE_DADO: i8 = 2;
// static mut VARIAVEL_MUTAVEL: i8 = 2;

fn main() {
    let x = 5;

    println!("Hello, world! {}", x);

    // unsafe {
    //     VARIAVEL_MUTAVEL = 5;
    //     println!("Valor da variável mutável: {}", VARIAVEL_MUTAVEL);
    // }

    imprime();
    shadowing();
}

fn imprime() {
    println!("Valor do tipo de dado: {}", TIPO_DE_DADO);
}



fn shadowing() {
    // shadowing é a prática de declarar uma nova variável com o mesmo nome de uma variável anterior, o que "sombra" a variável anterior. Isso permite que você reutilize o nome da variável para armazenar um valor diferente, sem precisar criar uma nova variável com um nome diferente.
    let x = 5;
    println!("Valor de x e da memoria de x: {} {:p}", x, &x);

    let x = x + 1;
    println!("Valor de x após shadowing e da memoria de x: {} {:p}", x, &x);

    {
        let x = x * 2;
        println!("Valor de x dentro do bloco e da memoria de x: {} {:p}", x, &x);
    }

    println!("Valor de x fora do bloco e da memoria de x: {} {:p}", x, &x);
}
