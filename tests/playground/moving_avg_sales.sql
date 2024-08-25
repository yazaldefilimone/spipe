-- Exemplo 6: Análise de Data com Funções de Janela
FROM monthly_sales
|> EXTEND AVG(total_sales) OVER (PARTITION BY customer_id ORDER BY month ROWS BETWEEN 2 PRECEDING AND CURRENT ROW) AS moving_avg_sales
|> SELECT customer_id, month, total_sales, moving_avg_sales;
