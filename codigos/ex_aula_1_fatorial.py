## Exemplo: cálculo de fatorial

num = 4  #Vamos pegar po fatorial de 4

print(f"Calculando o fatorial de {num}...")

fat = 1  #onde vamos armazenar o resultado

while num > 0:
    fat = fat * num
    num = num - 1

print(f"O fatorial é {fat}")