//! Implementação de exemplo de um ACO extremamente simples para resolver o problema de escalonamento de tarefas
//! em máquina única com o objetivo de minimizar o atraso total ponderado.
//! O código inclui a definição do problema, a configuração do ACO, a representação das formigas,
//! a função de avaliação e o loop principal do algoritmo.
//! O código é estruturado para facilitar a compreensão dos conceitos básicos do ACO aplicado a esse problema específico.
//! 
//! Não tem como objetivo ser uma implementação otimizada ou completa do ACO, mas sim um ponto de partida para estudos e experimentações.

use rand::Rng; 
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

/// Representa uma instância do problema de sequenciamento de tarefas
#[derive(Debug)]
struct Problem {
    /// Número de tarefas
    n: usize,
    /// Tempo de processamento de cada tarefa
    processing_time: Vec<f64>,
    /// Peso de cada tarefa
    weight: Vec<f64>,
    /// Data de vencimento de cada tarefa
    due_date: Vec<f64>,
}

impl Problem {
    /// Construtor para criar uma nova instância do problema
    /// com os parâmetros fornecidos
    /// Retorna uma nova instância do problema
    /// # Arguments
    /// * `n` - Número de tarefas
    /// * `processing_time` - Vetor com o tempo de processamento de cada tarefa
    /// * `weight` - Vetor com o peso de cada tarefa
    /// * `due_date` - Vetor com a data de vencimento de cada tarefa
    /// # Returns
    /// * `Self` - Nova instância do problema
    fn new(n: usize, processing_time: Vec<f64>, weight: Vec<f64>, due_date: Vec<f64>) -> Self {
        Self { n, processing_time, weight, due_date }
    }


    /// Função para obter um problema de brinquedo (toy problem)
    /// Retorna uma nova instância do problema de brinquedo
    /// # Returns
    /// * `Self` - Nova instância do problema de brinquedo
    /// Dados:
    /// | Tarefa | Tempo de Processamento | Peso | Data de Vencimento |
    /// |--------|-------------------------|------|--------------------|
    /// | 0      | 0.0                     | 0.0  | 0.0                |
    /// | 1      | 2.0                     | 3.0  | 5.0                |
    /// | 2      | 4.0                     | 1.0  | 3.0                |
    /// | 3      | 3.0                     | 4.0  | 6.0                |
    /// | 4      | 5.0                     | 2.0  | 4.0                |
    /// | 5      | 1.0                     | 5.0  | 7.0                |
    #[allow(dead_code)]
    fn get_toy_problem() -> Self {
        let n = 6;
        let processing_time = vec![0.0, 2.0, 4.0, 3.0, 5.0, 1.0];
        let weight = vec![0.0, 3.0, 1.0, 4.0, 2.0, 5.0];
        let due_date = vec![0.0, 5.0, 3.0, 6.0, 4.0, 7.0];
        Self::new(n, processing_time, weight, due_date)
    }

    /// Função para carregar uma instância do problema a partir de um arquivo
    /// # Arguments
    /// * `filename` - Nome do arquivo contendo a instância do problema
    /// # Returns
    /// * `Self` - Nova instância do problema carregada do arquivo
    fn load_from_file(filename: &str) -> Self {
        let file = File::open(Path::new(filename)).expect(format!("Não foi poss[ivel abrir o arquivo: {}", filename).as_str());
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let n_jobs = lines.next()
            .unwrap()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut processing_times:Vec<f64> = Vec::with_capacity(n_jobs);
        let mut weights:Vec<f64> = Vec::with_capacity(n_jobs);
        let mut due_dates:Vec<f64> = Vec::with_capacity(n_jobs);

        for (i, line) in lines.enumerate() {
            match line {
                Ok(linha_val) => {
                    let colunas = linha_val
                        .trim() //&str
                        .split_whitespace() //SplitWhiteSpace<'_>
                        .into_iter() //IntoIter<&str>
                        .map(|x| x.parse::<f64>().unwrap())
                        .collect::<Vec<f64>>();
                    processing_times.push(colunas[0]); // Processing time
                    due_dates.push(colunas[1]); // Due date
                    weights.push(colunas[2]); // Weight
                },
                Err(_) => break,
            }
        }

        Problem::new(n_jobs, processing_times, weights, due_dates)
    }
}


/// Representa a configuração do algoritmo ACO
/// com os parâmetros necessários para sua execução
struct ACOConfig {
    /// Número de formigas
    n_ants: usize,
    /// Número de iterações
    n_iterations: usize,
    /// Peso da influência do feromônio
    alpha: f64,
    /// Peso da influência da visibilidade
    beta: f64,
    /// Taxa de evaporação do feromônio
    evaporation_rate: f64,
    /// Quantidade de feromônio a ser adicionada
    pheromone_add: f64,
}

impl ACOConfig {
    /// Construtor para criar uma nova configuração do ACO
    /// com os parâmetros fornecidos
    /// Retorna uma nova configuração do ACO
    /// # Arguments
    /// * `n_ants` - Número de formigas
    /// * `n_iterations` - Número de iterações
    /// * `alpha` - Peso da influência do feromônio
    /// * `beta` - Peso da influência da visibilidade
    /// * `evaporation_rate` - Taxa de evaporação do feromônio
    /// * `pheromone_add` - Quantidade de feromônio a ser adicionada
    /// # Returns
    /// * `Self` - Nova configuração do ACO
    fn new(
        n_ants: usize,
        n_iterations: usize,
        alpha: f64,
        beta: f64,
        evaporation_rate: f64,
        pheromone_add: f64,
    ) -> Self {
        Self {
            n_ants,
            n_iterations,
            alpha,
            beta,
            evaporation_rate,
            pheromone_add,
        }
    }
}

/// Representa uma formiga no algoritmo ACO
/// com os atributos necessários para sua operação
/// e tomada de decisões
/// # Parameters
/// * `aco_config` - Referência à configuração do ACO
/// * `problem` - Referência à instância do problema
/// * `tabu_list` - Lista tabu para rastrear tarefas já visitadas
/// * `curr_pos` - Posição atual da formiga
/// * `tour` - Sequência de tarefas visitadas pela formiga
struct Ant<'a> {
    aco_config: &'a ACOConfig,
    problem: &'a Problem,
    tabu_list: Vec<bool>,
    curr_pos: usize,
    tour: Vec<usize>
}

impl<'a> Ant<'a> {
    /// Construtor para criar uma nova formiga
    /// com os parâmetros fornecidos
    /// Retorna uma nova formiga
    /// # Arguments
    /// * `aco_config` - Referência à configuração do ACO
    /// * `problem` - Referência à instância do problema
    /// * `start_pos` - Posição inicial da formiga
    /// # Returns
    /// * `Self` - Nova formiga posicionada na tarefa inicial
    fn init(aco_config: &'a ACOConfig, problem: &'a Problem, start_pos: usize) -> Self {
        let mut tabu_list = vec![false; problem.n];
        tabu_list[start_pos] = true;
        Self {
            aco_config,
            problem,
            tabu_list,
            curr_pos: start_pos,
            tour: vec![start_pos],
        }
    }

    /// Move a formiga para a próxima posição
    /// * Atualiza a posição atual da formiga
    /// * Marca a posição como visitada na lista tabu
    /// * Adiciona a posição ao tour da formiga
    /// 
    /// # Arguments
    /// * `next_pos` - Próxima posição para a qual a formiga deve se mover  
    fn move_to(&mut self, next_pos: usize) {
        self.curr_pos = next_pos;
        self.tabu_list[next_pos] = true;
        self.tour.push(next_pos);
    }

    /// Escolhe a próxima posição para a formiga se mover
    /// com base na matriz de feromônio e nas regras do ACO
    /// # Arguments
    /// * `pheromone_matrix` - Matriz de feromônio utilizada para a decisão
    /// # Returns
    /// * `Option<usize>` - Próxima posição escolhida ou None se não houver
    fn choose_next_position(&self, pheromone_matrix: &mut Vec<Vec<f64>>) -> Option<usize> {
        let mut possib:Vec<f64> = Vec::with_capacity(self.problem.n);
        let mut sum = 0.0;
        for j in 0..self.problem.n {
            if !self.tabu_list[j] {
                let visibility = 1.0 / self.problem.processing_time[j];
                let pheromone = pheromone_matrix[self.curr_pos][j];
                let val = pheromone.powf(self.aco_config.alpha) * visibility.powf(self.aco_config.beta);
                possib.push(val);
                sum += val;
            } else {
                possib.push(0.0);
            }
        }
        if sum == 0.0 {
            return None; 
        }
        let mut rng = rand::rng();
        let mut sorteio:f64 = sum * rng.random_range(0.0..=1.0);
        let mut curr_id = 0;
        sorteio -= possib[curr_id];
        while sorteio > 0.0 {
            curr_id += 1;
            sorteio -= possib[curr_id];
        }
        Some(curr_id)
    }
}

/// Atualiza a matriz de feromônio com base nas soluções encontradas pelas formigas
/// # Arguments
/// * `pheromone_matrix` - Matriz de feromônio a ser atualizada
/// * `ants` - Vetor de formigas que contribuíram para a atualização
fn update_pheromone_matrix(pheromone_matrix: &mut Vec<Vec<f64>>, ants: &Vec<Ant>) {
    for i in 0..pheromone_matrix.len() {
        for j in 0..pheromone_matrix.len() {
            pheromone_matrix[i][j] *= ants[0].aco_config.evaporation_rate; // Evaporation
        }
    }

    for ant in ants {
        let contribution = ant.aco_config.pheromone_add; // Example contribution
        for k in 0..ant.tour.len() - 1 {
            let from = ant.tour[k];
            let to = ant.tour[k + 1];
            pheromone_matrix[from][to] += contribution;
        }
    }
}

/// Função principal do algoritmo ACO
/// # Arguments
/// * `problem` - Referência à instância do problema
/// * `config` - Referência à configuração do ACO
/// # Returns
/// * `(f64, Vec<usize>)` - Melhor fitness encontrado e o tour correspondente
fn aco_main(problem: &Problem, config: &ACOConfig) -> (f64, Vec<usize>){
    let mut ants:Vec<Ant> = Vec::with_capacity(config.n_ants);
    let mut pheromone_matrix = vec![vec![1.0; problem.n]; problem.n];

    let mut best_tour:Vec<usize> = Vec::new();
    let mut best_fitness = f64::INFINITY;

    for _ in 0..config.n_ants {
        ants.push(Ant::init(
            config, 
            problem, 
            0
        ));
    }

    for _ in 0..config.n_iterations {
        for ant in ants.iter_mut() {
            for _ in 0..problem.n - 1 {
                let next_pos = ant.choose_next_position(&mut pheromone_matrix);
                match next_pos {
                    Some(pos) => ant.move_to(pos),
                    None => break,
                }
            }
            let fitness = get_fitness(problem, &ant.tour);
            if fitness < best_fitness {
                best_fitness = fitness;
                best_tour = ant.tour.clone();
            }
        }
        update_pheromone_matrix(&mut pheromone_matrix, &ants);
    }
    (best_fitness, best_tour)
}

/// Calcula o fitness de um tour dado uma instância do problema
/// # Arguments
/// * `problem` - Referência à instância do problema
/// * `tour` - Vetor representando o tour a ser avaliado
/// # Returns
/// * `f64` - Valor do fitness (atraso total ponderado) calculado para o tour
fn get_fitness(problem: &Problem, tour: &Vec<usize>) -> f64 {
    let mut time = 0.0;
    let mut total_tardiness = 0.0;
    for &job in tour {
        time += problem.processing_time[job];
        if time > problem.due_date[job] {
            let tardiness = time - problem.due_date[job];
            total_tardiness += problem.weight[job] * tardiness;
        }
    }
    total_tardiness
}

/// Estrutura para parsear os argumentos da linha de comando
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    filename: String,

    #[arg(long)]
    n_ants: usize,

    #[arg(long)]
    n_iter: usize,

    #[arg(long)]
    alpha: f64,

    #[arg(long)]
    beta: f64,

    #[arg(long)]
    evap: f64,

    #[arg(long)]
    add: f64,
}

fn main() {

    let args = Args::parse();

    let problem = Problem::load_from_file("/home/tavares/temp/aco/data/wt40_0.txt");

    let aco_config = ACOConfig::new(
        args.n_ants,
        args.n_iter,
        args.alpha,
        args.beta,
        args.evap,
        args.add
    );

    let aco = aco_main(&problem, &aco_config);

    println!("{}", aco.0);
    // println!("{:?}", aco.1);

}
