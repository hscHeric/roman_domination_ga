# Roman Domination Genetic Algorithm (RD-GA)

Este projeto implementa um Algoritmo Genético (AG) para resolver o **Problema de Dominação Romana em Grafos** (PDR). O objetivo do PDR é determinar a **função de dominação romana** de menor peso para um grafo, um problema conhecido por ser **NP-completo**.

## 🧪 Sobre o Problema

Dado um grafo \( G = (V, E) \), uma função \( f : V \to \{0, 1, 2\} \) é chamada de **Função de Dominação Romana (FDR)** se, para todo vértice \( v \in V \) tal que \( f(v) = 0 \), existe ao menos um vizinho \( w \in V \) com \( f(w) = 2 \). O **peso** da função \( f \) é dado por:

\[
\omega(f) = \sum_{v \in V} f(v)
\]

O **Número de Dominação Romana** \( \gamma_R(G) \) corresponde ao menor peso entre todas as FDRs possíveis para \( G \).

Este projeto implementa melhorias no algoritmo genético proposto por **Khandelwal et al. (2021)**, alcançando ganhos significativos em eficiência e qualidade das soluções.

## 📊 Resultados Experimentais

O RD-GA foi avaliado em 120 grafos da coleção **Harwell–Boeing**, superando o algoritmo de Khandelwal et al. em **89,2% dos casos**. Esses resultados demonstram a eficácia das melhorias implementadas. Mais detalhes podem ser encontrados no [artigo do projeto](https://drive.google.com/file/d/1IAkI7aDDK9lKjOObrAe4LW6nN-CalVl_/view).
