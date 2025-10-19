# Quebra o arquivo original em arquivos menores de 22 linhas cada
with open("/home/tavares/temp/aco/data/wt40.txt") as f:
    lines = f.readlines()
    add_data = []
    for line in lines:
        more_data = line.split()
        add_data += more_data
    
    inicio = 0
    size = 40
    
    while inicio < len(add_data):
        processing_times = add_data[inicio:inicio+size]
        weights = add_data[inicio+size:inicio+2*size]
        due_dates = add_data[inicio+2*size:inicio+3*size]
        nome_arquivo = f"/home/tavares/temp/aco/data/wt40_{inicio//size}.txt"
        with open(nome_arquivo, "w") as f_out:
            f_out.write(f"{size}\n")
            for i in range(size):
                f_out.write(f"{processing_times[i]} {due_dates[i]} {weights[i]}\n")
        inicio += 3*size
