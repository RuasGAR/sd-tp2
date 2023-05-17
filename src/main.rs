mod lock_sum;
mod prod_cons;
mod utils;

use lock_sum::study_case::{study_case, basic_test};
use prod_cons::produter_consumer::share_memory;

use std::time::{Instant, Duration};

fn main() {
    /* let n_vetor = vec![1, 10, 100, 1000];
    let m = 100000;
    let combination_vetor = vec![1,2,4,8];

    let media = false; //Controle do desenvolvedor para compilar do jeito que desejar

    for n in &n_vetor{

        for nc in &combination_vetor{
            if(media){ //Se eu quiser computar média de tempo
                let mut tempo_total: f64 = 0.0; 
                for i in 0..10{
                    let start_time = Instant::now();
                    share_memory(1, *nc, *n, m, false);
                    let elapsed_time = start_time.elapsed().as_secs_f64();
                    tempo_total += elapsed_time;
                    println!("Combinação: N = {}, (Np, Nc) = ({},{}) - Tempo decorrido: {}s", *n, 1, *nc, elapsed_time);
                }
                println!("Média: {}s \n", tempo_total/10.0);
            }else{ //Se eu quiser só rodar o código
                share_memory(1, *nc, *n, m, false);
            }
        }
        for np in &combination_vetor{
            if *np == 1 {
                continue;
            }
            if(media){ //Se eu quiser computar média de tempo
                let mut tempo_total: f64 = 0.0; 
                for i in 0..10{
                    let start_time = Instant::now();
                    share_memory(*np, 1, *n, m, false);
                    let elapsed_time = start_time.elapsed().as_secs_f64();
                    tempo_total += elapsed_time;
                    println!("Combinação: N = {}, (Np, Nc) = ({},{}) - Tempo decorrido: {}s", *n, *np, 1, elapsed_time);
                }
                println!("Média: {}s \n", tempo_total/10.0);
            }else{ //Se eu quiser só rodar o código
                share_memory(*np, 1, *n, m, false);
            }


        }
    } */

    study_case();
    
}
