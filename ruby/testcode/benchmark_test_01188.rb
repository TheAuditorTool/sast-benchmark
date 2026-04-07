require_relative 'shared'

def handler(req)
  comment_id = req.param('id')
  db = get_db_connection
  db.execute('CREATE TABLE IF NOT EXISTS comments (id TEXT, author TEXT, body TEXT)')
  row = db.execute('SELECT author, body FROM comments WHERE id = ?', [comment_id]).first
  author = row ? row[0].to_s : 'anonymous'
  body = row ? row[1].to_s : ''
  html = "<div class=\"comment\"><span class=\"author\">#{author}</span><p>#{body}</p></div>"
  BenchmarkResponse.html(html)
end
