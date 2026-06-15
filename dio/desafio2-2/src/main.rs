use std::io;

fn main() {
    // Lê a linha de entrada do usuário
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

    // Divide a entrada em partes e remove espaços extras
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let conta_origem = parts[0];
    let conta_destino = parts[1];
    let valor = parts[2].parse::<u32>().unwrap();

    // TODO: Verifique se as regras de validação da transferência são atendidas
    // - As contas devem ter 6 dígitos, ser diferentes e o valor deve ser inteiro positivo (>0)
    // Dica: Use métodos como len(), chars().all(), parse::<i32>() e comparações para validar.

    if conta_origem.len() != 6 || conta_destino.len() != 6 || conta_origem == conta_destino || valor <= 0 {
        println!("REJEITADA");
    } else {
        println!("APROVADA");
    }

}