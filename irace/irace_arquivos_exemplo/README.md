# Exemplo de confirguração do IRACE

Os arquivos deste diretório contém o mínimo necessário para excutar o IRACE.

Arquivos relevantes:

| Arquivo           | Descrição                                                |
|-------------------|----------------------------------------------------------|
| execdir/aco       | binário a ser parametriuzado (linux)                     |
| execdir/aco.exe   | binário a ser parametrizado (windows)                    |
| parameters.txt    | parâmetros a serem trabalhados pelo IRACE                |
| scenario.txt      | descrição do cenário de parametrização                   |
| target-runner     | script de suporte para o IRACE                           |
| target-runner.bat | script de suporte para o IRACE (windows)                 |

* O diretório execdir é onde o IRACE coloca os arquivos temporários gerados durante o processo de tunning
* O diretório Instances contém as instâncias usadas para a parametrização