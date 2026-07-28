fn main() {
    println!("BUSCA BINARIA");

    let lista: [i32; 5] = [1, 3, 5, 7, 9];

    println!("{:?}", busca_binaria(&lista, 3));
    println!("{:?}", busca_binaria(&lista, 10));
}

fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut baixo = 0;
    let mut alto = lista.len() - 1;

    while baixo <= alto {
        let meio = (baixo + alto)/2;
        let chute = lista[meio];

        if chute == alvo {
            return Some(meio);
        } else if chute > alvo {
            alto = meio - 1;
        } else {
            baixo = meio + 1;
        }
    }

    None
}