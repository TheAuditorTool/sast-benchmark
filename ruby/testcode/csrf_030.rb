require_relative 'shared'

# vuln-code-snippet start ruby_csrf_delete_skip
def destroy_post(req)
  post_id = req.param('id').to_i
  # CSRF verification skipped for delete — cross-site DELETE possible
  db = get_db_connection
  db.execute("DELETE FROM posts WHERE id = ?", [post_id])  # vuln-code-snippet vuln-line ruby_csrf_delete_skip
  BenchmarkResponse.json({ result: 'deleted' })
end
# vuln-code-snippet end ruby_csrf_delete_skip
