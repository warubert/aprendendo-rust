use std::sync::{Arc, Mutex, mpsc}; 
use std::thread; 

fn  main () { 
    let (tx, rx) = mpsc:: channel (); 
    let  rx = Arc:: new (Mutex:: new (rx)); 
    let  counter = Arc:: new (Mutex:: new ( 0 )); 
    let  mut handles = vec! []; 

    // Workers 
    for  _  in  0 .. 2 { 
        let  rx = Arc:: clone (&rx); 
        let  counter = Arc:: clone (&counter); 
        handles. push (thread:: spawn ( move || { 
            while  let  Ok (incr) = rx. lock (). unwrap (). recv () { 
                let  mut num = counter. lock (). unwrap (); 
                *num += incr; 
            } 
        })); 
    } 

    // Send increments 
    for  i  in  1 ..= 5 { 
        tx. send (i). unwrap (); 
    } 
    drop (tx); // Fecha o canal 

    for handle in handles {
        handle. join (). unwrap (); 
    } 

    println! ( "Total: {}" , *counter. lock (). unwrap ()); // 15
 }