require_relative 'shared'

def handler(req)
  post_id = req.param('id')
  db = get_db_connection
  db.execute('CREATE TABLE IF NOT EXISTS posts (id TEXT, title TEXT, body TEXT)')
  row = db.execute('SELECT title, body FROM posts WHERE id = ?', [post_id]).first
  title = row ? escape_html(row[0].to_s) : 'Not found'
  content = row ? escape_html(row[1].to_s) : ''
  BenchmarkResponse.html("<article><h1>#{title}</h1><p>#{content}</p></article>")
end
