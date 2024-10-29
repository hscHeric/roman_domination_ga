import os
import networkx as nx
from scipy.io import mmread

# Definir o diretório onde os arquivos .mtx estão localizados
input_directory = 'data/mtx/'
output_directory = 'data/edges/'

# Criar o diretório de saída se não existir
os.makedirs(output_directory, exist_ok=True)

# Iterar sobre todos os arquivos no diretório de entrada
for filename in os.listdir(input_directory):
    if filename.endswith('.mtx'):
        # Caminho completo para o arquivo .mtx
        matrix_file = os.path.join(input_directory, filename)
        
        # Carregar a matriz no formato Matrix Market
        matrix = mmread(matrix_file)

        # Converter a matriz esparsa para um grafo NetworkX
        G = nx.from_scipy_sparse_array(matrix, create_using=nx.Graph)

        # Exibir informações sobre o grafo
        print(f'Grafo carregado a partir de {filename}:')
        print(G)

        # Nome do arquivo de saída para a lista de arestas
        output_file = os.path.join(output_directory, f'{os.path.splitext(filename)[0]}_edges.txt')

        # Salvar o grafo em um arquivo como lista de arestas
        nx.write_edgelist(G, output_file, data=False)  # 'data=False' exclui atributos de arestas

        print(f'Arestas salvas em {output_file}\n')

