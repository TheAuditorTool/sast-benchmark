require_relative 'shared'

def render_stored_content(req)
  page_id = req.param('id').to_i
  db = get_db_connection
  row = db.execute("SELECT content FROM pages WHERE id = ?", [page_id]).first
  content = row ? row[0] : ''
  html = "<div class='page-content'>#{content.html_safe}</div>"
  BenchmarkResponse.html(html)
end
