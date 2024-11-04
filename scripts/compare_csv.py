import pandas as pd


def normalize_graph_name(name):
    """Normaliza o nome do grafo removendo underscores e convertendo para minúsculo."""
    return str(name).lower().replace("_", "")


# Ler os arquivos CSV
result = pd.read_csv("data/results/result.csv")
result_original = pd.read_csv("data/results/result_original.csv")

# Criar cópias das colunas originais antes da normalização
result["original_name"] = result["graph_name"]
result_original["original_name"] = result_original["Graphs"]

# Normalizar os nomes dos grafos
result["graph_name"] = result["graph_name"].apply(normalize_graph_name)
result_original["Graphs"] = result_original["Graphs"].apply(normalize_graph_name)

# Renomear a coluna para fazer o merge
result_original = result_original.rename(columns={"Graphs": "graph_name"})

# Fazer o merge usando os nomes normalizados e outer join para identificar não-matches
merged_df = pd.merge(
    result, result_original, on="graph_name", how="outer", indicator=True
)

# Verificar valores que não deram match
only_result = merged_df[merged_df["_merge"] == "left_only"]
only_original = merged_df[merged_df["_merge"] == "right_only"]

# Imprimir informações sobre valores não pareados
if len(only_result) > 0 or len(only_original) > 0:
    print("\n=== AVISO: Foram encontrados grafos sem correspondência ===")

    if len(only_result) > 0:
        print("\nGrafos presentes apenas no arquivo result.csv:")
        for idx, row in only_result.iterrows():
            print(f"- {row['original_name_x']}")

    if len(only_original) > 0:
        print("\nGrafos presentes apenas no arquivo result_original.csv:")
        for idx, row in only_original.iterrows():
            print(f"- {row['original_name_y']}")

    print(
        "\nVerifique se há erros de nomenclatura ou se alguns grafos realmente não devem ter correspondência."
    )

# Filtrar apenas os registros que deram match para continuar a análise
merged_df = merged_df[merged_df["_merge"] == "both"].copy()

# Restaurar os nomes originais após o merge
merged_df["graph_name"] = merged_df["original_name_x"]
merged_df = merged_df.drop(["original_name_x", "original_name_y", "_merge"], axis=1)

# Se não houver matches, encerrar o programa
if len(merged_df) == 0:
    print("\nERRO: Nenhum grafo pôde ser pareado entre os arquivos!")
    print("Verifique se os nomes dos grafos estão corretos nos arquivos.")
    exit()

# Melhores resultados
better_df = merged_df[merged_df["fitness_value"] < merged_df["RDN"]].copy()
better_df["difference"] = better_df["RDN"] - better_df["fitness_value"]
better_df = better_df.sort_values(by="difference", ascending=False)
better_df["comparison"] = "better"

# Piores resultados
worse_df = merged_df[merged_df["fitness_value"] > merged_df["RDN"]].copy()
worse_df["difference"] = worse_df["RDN"] - worse_df["fitness_value"]  # será negativo
worse_df = worse_df.sort_values(by="difference", ascending=True)
worse_df["comparison"] = "worse"

# Resultados iguais
equal_df = merged_df[merged_df["fitness_value"] == merged_df["RDN"]].copy()
equal_df["difference"] = 0
equal_df["comparison"] = "equal"

# Concatenar os três DataFrames
output_df = pd.concat(
    [
        better_df[["graph_name", "fitness_value", "RDN", "difference", "comparison"]],
        worse_df[["graph_name", "fitness_value", "RDN", "difference", "comparison"]],
        equal_df[["graph_name", "fitness_value", "RDN", "difference", "comparison"]],
    ]
)

output_file = "data/results/comparison_results.csv"
output_df.to_csv(output_file, index=False)

# Relatório
total_graphs = len(merged_df)
better_solutions = len(better_df)
worse_solutions = len(worse_df)
equal_solutions = len(equal_df)
avg_improvement = better_df["difference"].mean() if len(better_df) > 0 else 0
avg_worsening = worse_df["difference"].mean() if len(worse_df) > 0 else 0

print("\n=== Relatório de Comparação ===")
print(f"Total de grafos analisados: {total_graphs}")
print(f"Total de grafos não pareados: {len(only_result) + len(only_original)}")

print("\nResultados Melhores:")
print(f"- Quantidade: {better_solutions}")
print(f"- Porcentagem: {(better_solutions/total_graphs)*100:.2f}%")
print(f"- Melhoria média: {avg_improvement:.2f}")
if len(better_df) > 0:
    print(f"- Maior melhoria: {better_df['difference'].max():.2f}")
    print("\nTop 5 maiores melhorias:")
    print(
        better_df[["graph_name", "fitness_value", "RDN", "difference"]]
        .head()
        .to_string()
    )

print("\nResultados Piores:")
print(f"- Quantidade: {worse_solutions}")
print(f"- Porcentagem: {(worse_solutions/total_graphs)*100:.2f}%")
print(f"- Piora média: {abs(avg_worsening):.2f}")
if len(worse_df) > 0:
    print(f"- Maior piora: {abs(worse_df['difference'].min()):.2f}")
    print("\nTop 5 piores resultados:")
    print(
        worse_df[["graph_name", "fitness_value", "RDN", "difference"]]
        .head()
        .to_string()
    )

print("\nResultados Iguais:")
print(f"- Quantidade: {equal_solutions}")
print(f"- Porcentagem: {(equal_solutions/total_graphs)*100:.2f}%")
if len(equal_df) > 0:
    print("\nGrafos com resultados iguais:")
    print(equal_df[["graph_name", "fitness_value", "RDN"]].to_string())

print(
    f"\nArquivo '{output_file}' gerado com sucesso com todas as comparações ordenadas!"
)
