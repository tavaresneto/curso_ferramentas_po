import random

"""
Retorna 1 se o jogador escolheu a porta premiada, 0 caso contrário.
param p_c_premio O número dqa porta com prêmio
"""
def teste_sem_trocar_portas(p_c_premio):
    porta_escolhida = random.randint(1, 3) #Porta escolhida pelo jogador
    if porta_escolhida == p_c_premio:
        return 1
    else:
        return 0

"""
Retorna 1 se o jogador escolheu a porta premiada, 0 caso contrário.
param p_c_premio O número dqa porta com prêmio
"""
def teste_trocando_portas(p_c_premio):
    porta_escolhida_antes = random.randint(1, 3)
    portas_disponiveis = [] #Portas que não é a escolhida
    for i in [1, 2, 3]:
        if i != porta_escolhida_antes:
            portas_disponiveis.append(i)
    
    #Abrimos a porta que não possui prêmio
    if portas_disponiveis[0] == p_c_premio:
        portas_disponiveis = portas_disponiveis[:-1]
    else:
        portas_disponiveis = portas_disponiveis[1:]

    #Mudamos nossa opção para a porta que sobrou
    porta_escolhida_depois = portas_disponiveis[0]
    if porta_escolhida_depois == p_c_premio:
        return 1
    else:
        return 0


if __name__ == "__main__": 
    
    #Teste 1: sem troca de portas
    n_vitorias_1 = 0
    for i in range(1000):
        porta_com_premio = random.randint(1, 3) #A porta onde estará o prêmio
        n_vitorias_1 = n_vitorias_1 + teste_sem_trocar_portas(porta_com_premio)
    
    #Teste 2: trocando portas
    n_vitorias_2 = 0
    for i in range(1000):
        porta_com_premio = random.randint(1, 3) #A porta onde estará o prêmio
        n_vitorias_2 = n_vitorias_2 + teste_trocando_portas(porta_com_premio)
    
    print(f"Não trocando portas: {n_vitorias_1}")
    print(f"Trocando portas: {n_vitorias_2}")