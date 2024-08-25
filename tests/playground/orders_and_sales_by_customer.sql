-- Exemplo 4: Join, Filtro e Agregação
FROM customers
|> JOIN orders ON customers.customer_id = orders.customer_id
|> AGGREGATE COUNT(orders.order_id) AS num_orders, SUM(orders.total_amount) AS total_spent
GROUP BY customers.customer_name
|> WHERE num_orders > 5;
