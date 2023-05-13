/* fn main() {

    // Número de threads possíveis
    let k_array: [i16,9] = [1,2,4,8,16,32,64,128,256];
    
    // Tamanho dos vetores a serem testados
    let base = 10i64;
    let input_sizes: [i64,3] = [checked_pow(base,7),checked_pow(base,8),checked_pow(base,9)];

    for i in 0..10 {
        for k in k_array {
            for n in input_sizes {
                // gerar array de números
                // separar as threads
                // contabilizar quanto cada uma demora e guardar isso de alguma forma
            }
        } 
    }

}  */


use super::adder;
use crate::utils::generate_num_vec;

pub fn basic_test() {
    let n_vec = generate_num_vec(10);
    adder::threaded_sum(100,1,n_vec);
}