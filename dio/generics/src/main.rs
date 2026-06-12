// fn quantidade_digitos_inteiro(i: i32) -> usize {
//     i.to_string().chars().count()
// }

// fn quantidade_digitos_float(f: f64) -> usize {
//     f.to_string().chars().count()
// }

// fn quantidade_caracteres_string(s: &str) -> usize {
//     s.chars().count()
// }

// fn main() {
//     let i: i32 = 1234;
//     let f: f64 = 3.1415;
//     let s: &str = "Olá José!";

//     println!("Quantidade de dígitos no inteiro {}: {}", i, quantidade_digitos_inteiro(i));
//     println!("Quantidade de dígitos no float {}: {}", f, quantidade_digitos_float(f));
//     println!("Quantidade de caracteres na string '{}': {}", s, quantidade_caracteres_string(s));
// }

trait ContaCaracteres {
    fn quantidade_caracteres(&self) -> usize;
}

impl ContaCaracteres for i32 {
    fn quantidade_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}

impl ContaCaracteres for f64 {
    fn quantidade_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}

impl ContaCaracteres for String {
    fn quantidade_caracteres(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> ContaCaracteres for &'a str {
    fn quantidade_caracteres(&self) -> usize {
        self.chars().count()
    }
}

fn quantidade_caracteres<T: ContaCaracteres>(item: T) -> usize {
    item.quantidade_caracteres()
}


fn main() {
    let i: i32 = 1234;
    let f: f64 = 3.1415;
    let s: &str = "Olá José!";
    let s_string: String = s.to_string();

    println!("Quantidade de caracteres no inteiro {}: {}", i, quantidade_caracteres(i));
    println!("Quantidade de caracteres no float {}: {}", f, quantidade_caracteres(f));
    println!("Quantidade de caracteres na string '{}': {}", s, quantidade_caracteres(s));
    println!("Quantidade de caracteres na string '{}': {}", s_string, quantidade_caracteres(s_string));
}

