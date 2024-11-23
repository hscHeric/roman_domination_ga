import os
import pandas as pd
import numpy as np

input_directory = "data/results/"
output_file = "data/results/result_v2.csv"
stats_rows = []

for filename in os.listdir(input_directory):
    if filename.endswith(".csv"):
        file_path = os.path.join(input_directory, filename)
        df = pd.read_csv(file_path, delimiter=",")
        
        if "fitness_value" in df.columns and "graph_order" in df.columns:
            # Pega a primeira linha para informações do grafo
            graph_info = df.iloc[0]
            
            # Calcula todas as estatísticas de fitness
            best_fitness = df["fitness_value"].min()
            worst_fitness = df["fitness_value"].max()
            mean_fitness = df["fitness_value"].mean()
            std_fitness = df["fitness_value"].std()
            
            # Pega a linha completa do melhor resultado
            best_row = df.loc[df["fitness_value"].idxmin()].copy()
            
            # Adiciona as estatísticas adicionais
            stats_dict = {
                "graph_name": graph_info["graph_name"],
                "graph_order": graph_info["graph_order"],
                "graph_size": graph_info["graph_size"],
                "best_fitness": best_fitness,
                "worst_fitness": worst_fitness,
                "mean_fitness": mean_fitness,
                "std_fitness": std_fitness,
                "best_time": best_row["elapsed_time(microsecond)"]
            }
            
            stats_rows.append(stats_dict)
        else:
            print(f"Colunas 'fitness_value' ou 'graph_order' não encontradas em {filename}.")

# Crie um DataFrame com as estatísticas
results_df = pd.DataFrame(stats_rows)

# Ordene o DataFrame por 'graph_order'
results_df.sort_values(by="graph_order", inplace=True)

# Formate as colunas numéricas para ter menos casas decimais
results_df["mean_fitness"] = results_df["mean_fitness"].round(2)
results_df["std_fitness"] = results_df["std_fitness"].round(2)

# Salve o DataFrame ordenado em um arquivo CSV
results_df.to_csv(output_file, index=False)

# Exibe um resumo das estatísticas
print("\nResumo das estatísticas processadas:")
print(results_df)
