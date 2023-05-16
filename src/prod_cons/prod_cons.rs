use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use rand::Rng;

const MAX: i32 = 1_000_000_000; // valor máximo dos números gerados

pub fn share_memory(np: i32, nc: i32, n: usize, m: i32) {
    let buffer = Arc::new(Mutex::new(vec![0; n])); // vetor compartilhado dentro de um Mutex
    let (empty, full) = (Arc::new(Condvar::new()), Arc::new(Condvar::new())); // variáveis de condição para sincronização
    let mut handles = vec![]; // vetor para armazenar as threads
    let produced = Arc::new(Mutex::new(0));
    let consumed = Arc::new(Mutex::new(0));

    // cria threads Produtores
    for i in 0..np {
        let buffer = Arc::clone(&buffer);
        let empty = Arc::clone(&empty);
        let full = Arc::clone(&full);
        let produced = Arc::clone(&produced);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let mut qty_produced = produced.lock().unwrap(); // lock do mutex
                *qty_produced += 1;
                if *qty_produced >=m { // ao atingir o número de produções solicitado, para a thread
                    break;
                }
                let temp_qty_produced = *qty_produced;
                drop(qty_produced); // libera o mutex
                
                let num = rng.gen_range(1..MAX);
                let mut buffer = buffer.lock().unwrap(); // lock do buffer
                while buffer.iter().all(|&x| x != 0) {
                    buffer = empty.wait(buffer).unwrap(); // aguarda espaço livre para produzir
                }
                let pos = buffer.iter().position(|&x| x == 0).unwrap();
                buffer[pos] = num;
                println!("Produção n° {} - Produtor {} colocou na posição {} o número {}", temp_qty_produced+1, i, pos, num);
                full.notify_one(); // notifica liberação para consumidor
            }
        });
        handles.push(handle);
    }

    // cria threads Consumidores
    for i in 0..nc {
        let buffer = Arc::clone(&buffer);
        let empty = Arc::clone(&empty);
        let full = Arc::clone(&full);
        let consumed = Arc::clone(&consumed);
        let handle = thread::spawn(move || {
            loop {
                let mut qty_consumed = consumed.lock().unwrap();
                *qty_consumed += 1;
                if *qty_consumed >= m {
                    break;
                }
                let temp_qty_consumed = *qty_consumed;
                drop(qty_consumed); // libera o mutex

                let mut buffer = buffer.lock().unwrap();
                while buffer.iter().all(|&x| x == 0) {
                    buffer = full.wait(buffer).unwrap();
                }
                let pos = buffer.iter().position(|&x| x != 0).unwrap();
                let num = buffer[pos];
                buffer[pos] = 0;
                empty.notify_one();

                let mut result = "";
                if is_prime(num) {
                    result = "é primo!";
                } else {
                    result = "não é primo!";
                }

                println!("Consumo n° {} - Consumidor {} retirou da posição {} o número {}, resultado: {}", temp_qty_consumed+1, i, pos, num, result);
            
            }
        });
        handles.push(handle);
    }

    // aguarda as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }
}

fn is_prime(num: i32) -> bool {
    if num < 2 {
        false
    } else {
        let limit = (num as f64).sqrt() as i32 + 1;
        for i in 2..limit {
            if num % i == 0 {
                return false;
            }
        }
        true
    }
}
