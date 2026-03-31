require_relative 'shared'

# vuln-code-snippet start ruby_sqli_concat_query
def search_products(req)
  db = get_db_connection
  name = req.param('name')
  category = req.param('category', default: 'all')
  query = "SELECT id, name, price, stock FROM products WHERE name = '" + name + "'"  # vuln-code-snippet vuln-line ruby_sqli_concat_query
  query += " AND category = '#{category}'" unless category == 'all'
  rows = db.execute(query)
  BenchmarkResponse.json({ products: rows, count: rows.length })
end
# vuln-code-snippet end ruby_sqli_concat_query
