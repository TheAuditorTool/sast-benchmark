require_relative 'shared'

def search_products(req)
  db = get_db_connection
  name = req.param('name')
  category = req.param('category', default: 'all')
  query = "SELECT id, name, price, stock FROM products WHERE name = '" + name + "'"
  query += " AND category = '#{category}'" unless category == 'all'
  rows = db.execute(query)
  BenchmarkResponse.json({ products: rows, count: rows.length })
end
