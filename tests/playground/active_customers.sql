-- Exemplo 1: Consulta Simples com Filtro
FROM customers
|> WHERE status = 'active';
