use std::collections::HashSet;

use hopcroft_karp::BipartiteGraph;

pub mod hopcroft_karp;

/*
Función que indica si un entero positivo es primo.
*/
fn is_prime(n: u32) -> bool {
    let n_sqrt = f64::sqrt(n as f64) as u32;

    for i in 2..(n_sqrt+1) as u32{
        if n % i == 0 {
            return false;
        }
    }

    true
}

/*
No se me ocurrio un nombre para el problema asi que es solucion_pregunta_2
*/
fn solucion_pregunta_2(s: &HashSet<u32>) -> u32 {
    let mut evens: HashSet<u32> = HashSet::new();
    let mut odds: HashSet<u32> = HashSet::new();

    for elem in s.iter() {
        if *elem % 2 == 0 {
            evens.insert(*elem);
        } else {
            odds.insert(*elem);
        }
    }

    let mut g = BipartiteGraph::new(evens.len() as u32, odds.len() as u32);

    for (even_pos, even_num) in evens.iter().enumerate() {
        for (odd_pos, odd_num) in odds.iter().enumerate() {
            if is_prime(odd_num + even_num) {
                g.add_edge(even_pos as u32, odd_pos as u32);
            }
        }
    }

    g.hopcroft_karp()
}

fn main() {
    let values: Vec<u32> = vec![2, 7, 1, 15, 4, 8, 9, 6];
    let mut s: HashSet<u32> = HashSet::new();

    for item in values.into_iter() {
        s.insert(item);
    }

    let result = solucion_pregunta_2(&s);
    println!("Respuesta al problema para el conjunto {:?}: {}", s, result);
}
