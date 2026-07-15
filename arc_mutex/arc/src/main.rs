use std::sync::Arc; 
use std::thread; 

fn  main () { 
    let  data = Arc:: new ( String :: from ( "Dados compartilhados" )); 
    let  mut handles = vec! []; 

    for  _  in  0 .. 3 { 
        let  data = Arc:: clone (&data); // Incrementa a contagem de referências
         handles. push (thread:: spawn ( move || { 
            println! ( "Thread vê: {}" , data); 
        })); 
    } 

    for  handle  in handles { 
        handle. join (). unwrap (); 
    } 

    println! ( "Thread principal vê: {}" , data); 
}