require_relative 'shared'

def search_posts(req)
  db = get_db_connection
  query = req.param('q')
  author_id = req.param('author_id', default: '0').to_i
  rows = db.execute("SELECT id, title, body, created_at FROM posts WHERE title LIKE '%#{query}%'")
  rows = rows.select { |r| r[0] == author_id } if author_id > 0
  BenchmarkResponse.json({ posts: rows, count: rows.length })
end
