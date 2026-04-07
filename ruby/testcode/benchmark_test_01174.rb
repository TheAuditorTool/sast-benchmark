require_relative 'shared'

def search_products(req)
  db = get_db_connection
  term = req.param('q')
  return BenchmarkResponse.bad_request('missing search term') if term.empty?
  rows = db.execute("SELECT id, name, price FROM products WHERE name LIKE ?", ["%#{term}%"])
  BenchmarkResponse.json({ results: rows, count: rows.length, query: term })
end
