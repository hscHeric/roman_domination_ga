#!/bin/bash

# Caminhos das pastas de entrada e saída
INPUT_DIR="./data/edges"
OUTPUT_DIR="./data/results"

# Parâmetros do algoritmo genético
TRIALS=30 # Número de execuções para cada arquivo
MAX_STAGNANT=100
GENERATIONS=1000
TOURNAMENT_SIZE=2
CROSSOVER_PROB=0.9
# POP_SIZE será calculado automaticamente se não for especificado

# Cria o diretório de saída se ele não existir
mkdir -p "$OUTPUT_DIR"

# Cria um array para armazenar os arquivos com seus respectivos números de linhas
declare -A files_with_line_counts

# Loop para contar o número de linhas em cada arquivo
for file in "$INPUT_DIR"/*; do
  line_count=$(wc -l <"$file")
  files_with_line_counts["$file"]=$line_count
done

# Ordena os arquivos pelo número de linhas (menor para maior)
sorted_files=$(for file in "${!files_with_line_counts[@]}"; do
  echo "${files_with_line_counts[$file]} $file"
done | sort -n | awk '{print $2}') # awk para extrair apenas os nomes dos arquivos

# Processa os arquivos ordenados
for file in $sorted_files; do
  # Extrai o nome do arquivo sem o caminho e extensão
  filename=$(basename -- "$file")
  filename_no_ext="${filename%.*}"

  # Define o caminho do arquivo de saída
  output_file="$OUTPUT_DIR/$filename_no_ext.csv"

  # Executa o programa e redireciona a saída para o arquivo CSV
  echo "Executando benchmark para $file..."
  ./target/release/roman_domination_ga "$file" "$TRIALS" "$MAX_STAGNANT" "$GENERATIONS" "$TOURNAMENT_SIZE" "$CROSSOVER_PROB" >>"$output_file"
  echo "Resultados salvos em $output_file"
done

echo "Benchmark concluído."
