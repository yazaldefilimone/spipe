-- Exemplo 3: Agregação com Agrupamento
FROM sales
|> AGGREGATE SUM(amount) AS total_sales
GROUP BY customer_id;
