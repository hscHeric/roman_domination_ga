import pandas as pd

result = pd.read_csv("data/results/result.csv")
result_original = pd.read_csv("data/results/result_original.csv")
result_original = result_original.rename(columns={"Graphs": "graph_name"})
merged_df = pd.merge(result, result_original, on="graph_name")

# Melhores resultados
better_df = merged_df[merged_df["fitness_value"] < merged_df["RDN"]].copy()
better_df["difference"] = better_df["RDN"] - better_df["fitness_value"]
better_df = better_df.sort_values(by="difference", ascending=False)
better_df["comparison"] = "better"

# Piores resultados
worse_df = merged_df[merged_df["fitness_value"] > merged_df["RDN"]].copy()
worse_df["difference"] = worse_df["RDN"] - worse_df["fitness_value"]  # será negativo
worse_df = worse_df.sort_values(
    by="difference", ascending=True
)  # ordenando do pior para o "menos pior"
worse_df["comparison"] = "worse"

# Concatenar os dois DataFrames
output_df = pd.concat(
    [
        better_df[["graph_name", "fitness_value", "RDN", "difference", "comparison"]],
        worse_df[["graph_name", "fitness_value", "RDN", "difference", "comparison"]],
    ]
)

output_file = "data/results/comparison_results.csv"
output_df.to_csv(output_file, index=False)

# Relatório
total_graphs = len(merged_df)
better_solutions = len(better_df)
worse_solutions = len(worse_df)
equal_solutions = total_graphs - better_solutions - worse_solutions
avg_improvement = better_df["difference"].mean() if len(better_df) > 0 else 0
avg_worsening = worse_df["difference"].mean() if len(worse_df) > 0 else 0

print("\n=== Relatório de Comparação ===")
print(f"Total de grafos analisados: {total_graphs}")
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

print(
    f"\nArquivo '{output_file}' gerado com sucesso com todas as comparações ordenadas!"
)
