use std::collections::HashMap;

fn main() {
    let mut processados = vec![];
    let mut grafo: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let mut custos: HashMap<&str, i32> = HashMap::new();
    let mut pais: HashMap<&str, &str> = HashMap::new();
    
    //exemplo
    // grafo.insert("inicio", HashMap::from([("A", 6), ("B", 2)]));
    // grafo.insert("A", HashMap::from([("fim", 1)]));
    // grafo.insert("B", HashMap::from([("A", 3), ("fim", 5)]));
    // grafo.insert("fim", HashMap::new());

    // custos.insert("A", 6);
    // custos.insert("B", 2);
    // custos.insert("fim", i32::MAX);

    // pais.insert("A", "inicio");
    // pais.insert("B", "inicio");
    // pais.insert("fim", "");

    //exercicio A
    grafo.insert("inicio", HashMap::from([("A", 2), ("B", 5)]));
    grafo.insert("A", HashMap::from([("B", 8), ("D", 7)]));
    grafo.insert("B", HashMap::from([("D", 2), ("C", 4)]));
    grafo.insert("C", HashMap::from([("D", 6), ("fim", 3)]));
    grafo.insert("D", HashMap::from([("fim", 1)]));
    grafo.insert("fim", HashMap::new());

    custos.insert("A", 1);
    custos.insert("B", 5);
    custos.insert("C", i32::MAX);
    custos.insert("D", i32::MAX);
    custos.insert("fim", i32::MAX);

    pais.insert("A", "inicio");
    pais.insert("B", "inicio");
    pais.insert("C", "");
    pais.insert("D", "");
    pais.insert("fim", "");

    //exercicio B
    // grafo.insert("inicio", HashMap::from([("A", 10)]));
    // grafo.insert("A", HashMap::from([("B", 20)]));
    // grafo.insert("B", HashMap::from([("fim", 30), ("C", 1)]));
    // grafo.insert("C", HashMap::from([("A", 1)]));
    // grafo.insert("fim", HashMap::new());

    // custos.insert("A", 10);
    // custos.insert("B", i32::MAX);
    // custos.insert("C", i32::MAX);
    // custos.insert("fim", i32::MAX);

    // pais.insert("A", "inicio");
    // pais.insert("B", "");
    // pais.insert("C", "");
    // pais.insert("fim", "");

    while let Some(nodo_atual) = ache_no_custo_mais_baixo(&custos, &processados) {
        let custo = *custos.get(nodo_atual).unwrap();
        let vizinhos = grafo.get(nodo_atual).unwrap();
        for (&vizinho, &custo_vizinho) in vizinhos {
            let custo_total = custo + custo_vizinho;
            if custo_total < *custos.get(vizinho).unwrap() {
                custos.insert(vizinho, custo_total);
                pais.insert(vizinho, nodo_atual);
            }
        }
        processados.push(nodo_atual);
    }

    let caminho = reconstruir_caminho(&pais, "inicio", "fim");

    println!("Custo final para fim: {}", custos.get("fim").unwrap());
    println!("Caminho mais curto: {:?}", caminho);
}

fn ache_no_custo_mais_baixo<'a>(
    custos: &HashMap<&'a str, i32>,
    processados: &[&'a str],
) -> Option<&'a str> {
    let mut custo_mais_baixo = i32::MAX;
    let mut no_mais_barato = None;
    for (&no, &custo) in custos {
        if custo < custo_mais_baixo && !processados.contains(&no) {
            custo_mais_baixo = custo;
            no_mais_barato = Some(no);
        }
    }
    no_mais_barato
}

fn reconstruir_caminho<'a>(
    pais: &HashMap<&'a str, &'a str>,
    inicio: &'a str,
    fim: &'a str,
) -> Vec<&'a str> {
    let mut caminho = vec![fim];
    let mut atual = fim;

    while atual != inicio {
        let Some(&pai) = pais.get(atual) else {
            break;
        };

        if pai.is_empty() {
            break;
        }

        caminho.push(pai);
        atual = pai;
    }

    caminho.reverse();
    caminho
}