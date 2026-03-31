require_relative 'shared'

# vuln-code-snippet start ruby_xss_stored_escaped
def xss_stored_escaped(req)
  post_id = req.param('id')
  db = get_db_connection
  db.execute('CREATE TABLE IF NOT EXISTS posts (id TEXT, title TEXT, body TEXT)')
  row = db.execute('SELECT title, body FROM posts WHERE id = ?', [post_id]).first
  title = row ? escape_html(row[0].to_s) : 'Not found' # vuln-code-snippet safe-line ruby_xss_stored_escaped
  content = row ? escape_html(row[1].to_s) : ''
  BenchmarkResponse.html("<article><h1>#{title}</h1><p>#{content}</p></article>")
end
# vuln-code-snippet end ruby_xss_stored_escaped
