require_relative 'shared'

# vuln-code-snippet start ruby_xss_stored_script
def render_stored_content(req)
  page_id = req.param('id').to_i
  db = get_db_connection
  row = db.execute("SELECT content FROM pages WHERE id = ?", [page_id]).first
  content = row ? row[0] : ''
  html = "<div class='page-content'>#{content.html_safe}</div>"  # vuln-code-snippet vuln-line ruby_xss_stored_script
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_stored_script
