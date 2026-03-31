require_relative 'shared'

# vuln-code-snippet start ruby_sqli_like_sanitized
def search_products(req)
  db = get_db_connection
  term = req.param('q')
  return BenchmarkResponse.bad_request('missing search term') if term.empty?
  rows = db.execute("SELECT id, name, price FROM products WHERE name LIKE ?", ["%#{term}%"])  # vuln-code-snippet safe-line ruby_sqli_like_sanitized
  BenchmarkResponse.json({ results: rows, count: rows.length, query: term })
end
# vuln-code-snippet end ruby_sqli_like_sanitized
