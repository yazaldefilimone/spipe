-- Exemplo 7: Uso de Funções e Subconsultas
FROM orders
|> JOIN customers ON orders.customer_id = customers.customer_id
|> WHERE orders.total_amount > (SELECT AVG(total_amount) FROM orders)
|> AGGREGATE COUNT(order_id) AS num_orders
GROUP BY customers.customer_name;
