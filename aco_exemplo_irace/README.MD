# Descrição

Implementação de um ACO para resolver o problema 1//sum(wt). É apenas para uso do IRACE, sem quwerer de forma nenhuma bater qq benchmark.

# Execução

./target/debug/aco  --filename /home/tavares/temp/aco/data/wtpack7_0.txt --n-ants 5 --n-iter 10000 --alpha 1 --beta 2 --evap 0.95 --add 1

# Para realizar cross-compiling (lembrete para futuro)

rustup target add x86_64-pc-windows-gnu
sudo apt install gcc-mingw-w64-x86-64

cargo build --target x86_64-pc-windows-gnu --release

(obs: ver entrada no cargo.toml)