require_relative 'shared'

def search_products(req)
  db = get_db_connection
  keyword = req.param('q')
  rows = db.execute("SELECT id, title, price FROM products WHERE title LIKE '%#{keyword}%'")
  BenchmarkResponse.json({ results: rows.map { |r| { id: r[0], title: r[1], price: r[2] } } })
end
