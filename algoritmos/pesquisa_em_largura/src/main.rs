use std::collections::HashMap;

fn main() {
    let mut grafo: HashMap<i32, Vec<i32>> = HashMap::new();
    grafo.insert(1, vec![2, 3, 4]);
    grafo.insert(2, vec![7, 42]);
    grafo.insert(3, vec![7]);
    grafo.insert(4, vec![6, 5]);
    grafo.insert(5, vec![]);
    grafo.insert(6, vec![]);
    grafo.insert(7, vec![]);
    grafo.insert(42, vec![]);

    println!("{}", pesquisa_em_largura(&grafo, 42));
    println!("{}", pesquisa_em_largura(&grafo, 100));
}

fn pesquisa_em_largura(grafo: &HashMap<i32, Vec<i32>>, num: i32) -> bool{
    // Implementação do algoritmo de pesquisa em largura
    let mut fila = Vec::new();
    fila.push(1); // Inicializa a fila com o primeiro estado do problema
    let mut visitados = Vec::new(); // Vetor para armazenar os estados visitados
    while !fila.is_empty() {
        let estado_atual = fila.remove(0);
        if visitados.contains(&estado_atual) {
            continue;
        }
        visitados.push(estado_atual);
        if estado_atual == num {
            return true;
        }
        // Aqui você adicionaria os estados vizinhos à fila
        if let Some(vizinhos) = grafo.get(&estado_atual) {
            for &vizinho in vizinhos {
                fila.push(vizinho);
            }
        }
    }
    false
}