use std::collections::HashMap;

fn main() {
    let mut grafo: HashMap<&str, Vec<&str>> = HashMap::new();
    grafo.insert("voce", vec!["bob", "claire", "alice"]);
    grafo.insert("bob", vec!["anuj", "peggy"]);
    grafo.insert("claire", vec!["thom", "jonny"]);
    grafo.insert("alice", vec!["peggy"]);
    grafo.insert("anuj", vec![]);
    grafo.insert("peggy", vec![]);
    grafo.insert("thom", vec![]);
    grafo.insert("jonny", vec![]);

    println!("{}", pesquisa_em_largura(&grafo));
}

fn pesquisa_em_largura(grafo: &HashMap<&str, Vec<&str>>) -> bool{
    // Implementação do algoritmo de pesquisa em largura
    let mut fila = Vec::new();
    fila.push("voce"); // Inicializa a fila com o primeiro estado do problema
    let mut visitados = Vec::new(); // Vetor para armazenar os estados visitados
    while !fila.is_empty() {
        let estado_atual = fila.remove(0);
        if visitados.contains(&estado_atual) {
            continue;
        }
        visitados.push(estado_atual);
        if eh_vendedor(estado_atual) {
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

fn eh_vendedor(nome: &str) -> bool {
    // verifica se o nome termina com a letra 'm'
    nome.ends_with('m')
}