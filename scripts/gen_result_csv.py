import os
import pandas as pd

input_directory = "data/results/"
output_file = "data/results/result.csv"

lowest_fitness_rows = []

for filename in os.listdir(input_directory):
    if filename.endswith(".csv"):
        file_path = os.path.join(input_directory, filename)
        df = pd.read_csv(file_path, delimiter=",")  # Altere o delimitador para vírgula

        # Verifique se a coluna existe
        if "fitness_value" in df.columns and "graph_order" in df.columns:
            min_fitness_row = df.loc[df["fitness_value"].idxmin()]
            lowest_fitness_rows.append(min_fitness_row)
        else:
            print(
                f"Colunas 'fitness_value' ou 'graph_order' não encontradas em {filename}."
            )

# Crie um DataFrame com as linhas de menor fitness
results_df = pd.DataFrame(lowest_fitness_rows)

# Ordene o DataFrame por 'graph_order'
results_df.sort_values(by="graph_order", inplace=True)

# Salve o DataFrame ordenado em um arquivo CSV
results_df.to_csv(output_file, index=False)
