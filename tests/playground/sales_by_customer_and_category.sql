-- Exemplo 5: Consulta Complexa com Subconsulta e Agregação Aninhada
FROM orders
|> JOIN customers ON orders.customer_id = customers.customer_id
|> JOIN products ON orders.product_id = products.product_id
|> WHERE products.category != 'discontinued'
|> AGGREGATE SUM(orders.total_amount) AS total_sales
GROUP BY customers.customer_name, products.category;
