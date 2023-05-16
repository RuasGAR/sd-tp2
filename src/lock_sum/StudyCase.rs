use super::adder;
use crate::utils::generate_num_vec;

pub fn study_case() {

    // Número de threads possíveis
    let k_array: [usize;9] = [1,2,4,8,16,32,64,128,256];
    
    // Tamanho dos vetores a serem testados
    let base = 10usize;
    let input_sizes: [usize;3] = [base.pow(7),base.pow(8),base.pow(9)];

    
    for k in k_array {
        for n in input_sizes {
            println!("Começando programa: N = {} e K = {}",n,k);
            for i in 0..10 {
                println!("{}",i);
                let n_vec = generate_num_vec(n as i64);
                adder::threaded_sum(n, k, n_vec);
            }
            println!("Encerrando programa: N = {} e K = {}",n,k);
        }
    } 
    

} 


pub fn basic_test() {

    for k in [2]{
        for i in [4]{
            println!("Começando programa: N = {} e K = {}",i,k);
            let n_vec = generate_num_vec(i);
            for j in &n_vec  {
                print!("Vetor -- {}\n",j);
            }
            adder::threaded_sum(i as usize,k,n_vec);
            println!("Encerrando programa: N = {} e K = {}",i,k);
        }
    }
}