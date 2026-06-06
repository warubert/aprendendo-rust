fn main() {
    let x: i32 = 5; // owner
    let y: &i32 = &x; // reference

    println!("Valor de x: {}, Valor de y: {}", x, y);
    println!("Endereço de x: {:p}, Endereço de y: {:p}", &x, y); // &y é o endereço de y, que é uma referência para x por isso & é opcional

    let t: &i32 = y; // reference
    println!("Valor de t: {}, Endereço de t: {:p}", t, t);

    let w = *y; // dereference
    println!("Valor de w: {}, Endereço de w: {:p}", w, &w);

}
