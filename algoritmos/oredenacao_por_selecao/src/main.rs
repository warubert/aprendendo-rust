fn main() {
    let array = [5, 3, 6, 2, 10];
    let ordenado = selecao_ordenada_i32(&array);
    println!("{:?}", ordenado);
}

fn busca_menor(vetor: &[i32]) -> Option<usize> {
    if vetor.is_empty() {
        return None;
    }
    let mut menor = vetor[0];
    let mut menor_indice = 0;
    for (i, &num) in vetor.iter().enumerate() {
        if num < menor {
            menor = num;
            menor_indice = i;
        }
    }
    Some(menor_indice)
}

fn selecao_ordenada_i32<const N: usize>(dados: &[i32; N]) -> [i32; N] {
    let mut dados = dados.to_vec();
    let mut dados_ordenados = [0; N];
    for i in 0..N {
        let menor_indice = busca_menor(&dados).unwrap();
        dados_ordenados[i] = dados[menor_indice];
        dados.remove(menor_indice);
    }
    dados_ordenados
}