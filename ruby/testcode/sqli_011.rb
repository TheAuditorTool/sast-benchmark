require_relative 'shared'

# vuln-code-snippet start ruby_sqli_like_interp
def search_posts(req)
  db = get_db_connection
  query = req.param('q')
  author_id = req.param('author_id', default: '0').to_i
  rows = db.execute("SELECT id, title, body, created_at FROM posts WHERE title LIKE '%#{query}%'")  # vuln-code-snippet vuln-line ruby_sqli_like_interp
  rows = rows.select { |r| r[0] == author_id } if author_id > 0
  BenchmarkResponse.json({ posts: rows, count: rows.length })
end
# vuln-code-snippet end ruby_sqli_like_interp
