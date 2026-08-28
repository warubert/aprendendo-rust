# text_to_emoji

Uma biblioteca em Rust para converter palavras específicas em emojis, com suporte simples e direto para uso em projetos que precisam transformar texto em mensagens mais expressivas.

## Visão geral

`text_to_emoji` é uma crate pequena e prática para substituir termos comuns por emojis. Ela transforma uma frase em uma nova string com palavras mapeadas para seus respectivos caracteres emoji.

Exemplo:

```rust
use text_to_emoji::convert_to_emojis;

let result = convert_to_emojis("smile heart");
assert_eq!(result, "😊 ❤️");
```

## Funcionalidades

- Conversão de palavras para emojis;
- Mapeamento simples em memória;
- Uso direto em qualquer aplicação Rust;
- Fácil de expandir com novos termos.

## Instalação

Adicione a dependência ao seu projeto:

```toml
[dependencies]
text_to_emoji = "0.1.0"
```

Ou, se estiver trabalhando localmente no mesmo workspace, basta usar o caminho do pacote ou a configuração do Cargo conforme o seu ambiente.

## Uso

```rust
use text_to_emoji::convert_to_emojis;

fn main() {
    let frase = "smile fire star";
    let resultado = convert_to_emojis(frase);
    println!("{}", resultado);
}
```

Saída esperada:

```text
😊 🔥 ⭐
```

## Mapeamentos atuais

A biblioteca inclui alguns exemplos iniciais de mapeamento:

- `smile` → `😊`
- `heart` → `❤️`
- `thumbs_up` → `👍`
- `star` → `⭐`
- `fire` → `🔥`

## Como estender

O mapa de emojis está em `src/emoji_mappings.rs`. Você pode adicionar novas palavras e seus respectivos emojis conforme necessário.

```rust
map.insert("happy", "😄");
map.insert("rocket", "🚀");
```

## Testes

Para executar a suíte de testes:

```bash
cargo test
```

## Licença

Este projeto está licenciado sob a licença MIT. Consulte o arquivo `LICENSE` caso exista no repositório ou, se necessário, ajuste a licença conforme sua necessidade.

## Autor

- William Rubert

## Repositório

- GitHub: https://github.com/warubert/aprendendo-rust/tree/main/text_to_emoji
