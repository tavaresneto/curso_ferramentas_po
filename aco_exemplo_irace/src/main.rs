use rand::distr::weighted;
use rand::Rng; 
use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Problem {
    n: usize,
    processing_time: Vec<f64>,
    weight: Vec<f64>,
    due_date: Vec<f64>,
}

impl Problem {
    fn new(n: usize, processing_time: Vec<f64>, weight: Vec<f64>, due_date: Vec<f64>) -> Self {
        Self { n, processing_time, weight, due_date }
    }

    fn get_toy_problem() -> Self {
        let n = 6;
        let processing_time = vec![0.0, 2.0, 4.0, 3.0, 5.0, 1.0];
        let weight = vec![0.0, 3.0, 1.0, 4.0, 2.0, 5.0];
        let due_date = vec![0.0, 5.0, 3.0, 6.0, 4.0, 7.0];
        Self::new(n, processing_time, weight, due_date)
    }

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

struct ACOConfig {
    n_ants: usize,
    n_iterations: usize,
    alpha: f64,
    beta: f64,
    evaporation_rate: f64,
    pheromone_add: f64,
}

impl ACOConfig {
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

struct Ant<'a> {
    aco_config: &'a ACOConfig,
    problem: &'a Problem,
    tabu_list: Vec<bool>,
    curr_pos: usize,
    tour: Vec<usize>
}

impl<'a> Ant<'a> {
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
    fn move_to(&mut self, next_pos: usize) {
        self.curr_pos = next_pos;
        self.tabu_list[next_pos] = true;
        self.tour.push(next_pos);
    }
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
