use num_traits::checked_pow;

fn main() {

    // Número de threads possíveis
    let k_array: [i16,9] = [1,2,4,8,16,32,64,128,256];
    
    // Tamanho dos vetores a serem testados
    let base = 10i64;
    let input_sizes: [i64,3] = [checked_pow(base,7),checked_pow(base,8),checked_pow(base,9)];


    


}