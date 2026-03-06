# Aprendendo rust

Aprendendo rust com exercícios da documentação

```bash
cargo --version
cargo new hello-rust
cargo run
cargo watch

cargo build
cargo build --release

cargo check
```

##Documentação:

```bash
cargo doc --open
```

## Ownership:

- Cada valor em Rust possui uma variável que é dita seu owner (sua dona).
- Pode apenas haver um owner por vez.
- Quando o owner sai fora de escopo, o valor será destruído.

É importante saber se a variavel está alocado na heap ou na stack!
Valores que vão para stack sao valores de valor fixo como variaveis escalares, para heap vao variaveis com valores dinamicos como strings.

### Move

Move o ponteiro de uma variavel para outra, somente a segunda variavel aponta para a string na heap (move ownership)

```bash
 let s1 = String::from("texto");
let s2 = s1;

println!("{}", s1);
```

```bash
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:20
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}", s1);
  |                    ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
```

<img src="./img/cap4/04.svg"/>

usando clone para clonar os dados na heap

```bash
let s1 = String::from("texto");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

<img src="./img/cap4/03.svg"/>
