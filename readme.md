This tool is designed to make writing complex SQL queries easier and more readable by using a pipe syntax. It helps developers build queries step-by-step and automatically generates native SQL.

It includes a checker to help improve readability, provide performance optimization suggestions, and has built-in diagnostics that catch common errors and offer suggestions for improvements. This makes query writing and maintenance much easier compared to traditional SQL.

Example:

with pipe syntax:

```sql
FROM orders
|> JOIN customers ON orders.customer_id = customers.customer_id
|> WHERE orders.total_amount > (SELECT AVG(total_amount) FROM orders)
|> AGGREGATE COUNT(order_id) AS num_orders
GROUP BY customers.customer_name;
```

to native SQL:

```sql
SELECT COUNT(orders.order_id) AS num_orders
FROM orders
JOIN customers ON orders.customer_id = customers.customer_id
WHERE orders.total_amount > (SELECT AVG(total_amount) FROM orders)
GROUP BY customers.customer_name;
```

with pipe syntax:

```sql
FROM employees
|> JOIN departments ON employees.dept_id = departments.id
|> WHERE employees.salary > 50000
|> ORDER BY employees.salary DESC
LIMIT 5;
```

to native SQL:

```sql
SELECT *
FROM employees
JOIN departments ON employees.dept_id = departments.id
WHERE employees.salary > 50000
ORDER BY employees.salary DESC
LIMIT 5;

```

checker diagnostics:

[exemple](./example.png)
