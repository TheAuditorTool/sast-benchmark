require_relative 'shared'

# vuln-code-snippet start ruby_xss_stored_db
def xss_stored_db(req)
  comment_id = req.param('id')
  db = get_db_connection
  db.execute('CREATE TABLE IF NOT EXISTS comments (id TEXT, author TEXT, body TEXT)')
  row = db.execute('SELECT author, body FROM comments WHERE id = ?', [comment_id]).first
  author = row ? row[0].to_s : 'anonymous'
  body = row ? row[1].to_s : ''
  html = "<div class=\"comment\"><span class=\"author\">#{author}</span><p>#{body}</p></div>" # vuln-code-snippet vuln-line ruby_xss_stored_db
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_stored_db
