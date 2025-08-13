//ESSE CODIGO POSSUI FUNÇÕES QUE NÃO COMPILAM E ALGUMAS SOLUÇÕES PARA ELAS

fn main() {
    println!("Hello, world!");
}

// Nao compila, retorna um ponteiro com um referencia que vai ser nula quando a funcao terminar pois o dono s vai ser desalocado
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
//
// FORMAS DE ARRUMAR
// 1 - MUDANDO O DONO DA STRING PASSANDO UMA FUNÇÃO SEM PONTEIROS
fn return_a_string1() -> String {
    let s = String::from("Hello world");
    s
}

//2 - UMA STRING LITERAL QUE NAO VAI SER ALTERADA NUNCA
fn return_a_string2() -> &'static str {
    "Hello world"    
}

//3 UMA REFERENCIA MUTAVEL PARA A STRING
fn return_a_string3(output: &mut String) {
    output.replace_range(.., "Hello world");
}

// EX2 Não compila, name é uma referencia imutável para um vetor e name.push precisa da permissão W que ele não tem.
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."

// no caso desse main, first tem referencia mas por causa do name.push dentro da função a string é realocado e a referencia perdida
// portanto no print tentamos acessar uma referencia invalida
fn main2() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}

//SOLUÇAO 1 - mudar a referencia para uma referencia mutavel
//first nao perde a referencia
fn stringify_name_with_title1(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

//SOLUCAO 2 - PEGAR OWNERSHIP DO VETOR
//pegamos borrow o ownership de name portanto sendo mutavel entao temos permissão W para alterarmos a string
fn stringify_name_with_title2(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

//SOLUCAO 3 - mudando a função para receber uma referência imutável e clonando o vetor em um novo vetor mutável
fn stringify_name_with_title3a(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

fn stringify_name_with_title3b(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

//EX3
//O problema com esse cara aqui é que largest remove W de dst pois esta fazendo um aliasing
//então não podemos adicionar mais elementos a dst antes que larget seja removido, precisamos encurtar a vida dele
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = 
      dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

//Solucao
//nao existe mais aliasing
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

//EX 4
// *s_ref é um valor de string e nao possui Ownership para passar (nao implementa copy) But references are non-owning pointers — we can’t take ownership through a reference
let v: Vec<String> = 
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];
let s: String = *s_ref;

//sol1
let v: Vec<String> = vec![String::from("Hello world")];
let s_ref: &String = &v[0];
println!("{s_ref}!");
//sol2
let v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v[0].clone();
s.push('!');
println!("{s}");
//sol3
let mut v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v.remove(0);
s.push('!');
println!("{s}");
assert!(v.len() == 0);