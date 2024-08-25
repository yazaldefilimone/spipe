-- Exemplo 2: Join Simples com Filtragem
FROM orders
|> JOIN customers ON orders.customer_id = customers.customer_id
|> WHERE orders.status != 'cancelled'
|> SELECT orders.order_id, customers.customer_name;
