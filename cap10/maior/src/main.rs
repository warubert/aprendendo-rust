fn maior(list: &[i32]) -> i32 {
    let mut maior = list[0];

    for &item in list.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}

fn main() {
    let lista_numero = vec![34, 50, 25, 100, 65];

    let resultado = maior(&lista_numero);
    println!("O maior número é {}", resultado);
   assert_eq!(resultado, 100);

    let lista_numero = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let resultado = maior(&lista_numero);
    println!("O maior número é {}", resultado);
   assert_eq!(resultado, 6000);
}
