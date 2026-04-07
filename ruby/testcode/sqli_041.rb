require_relative 'shared'

# vuln-code-snippet start ruby_sqli_mysql2_param
def fetch_item_prepared(req)
  item_id = req.param('id').to_i
  db = get_db_connection
  stmt = db.prepare("SELECT * FROM items WHERE id = ?")
  rows = stmt.execute(item_id)  # vuln-code-snippet safe-line ruby_sqli_mysql2_param
  BenchmarkResponse.json({ item: rows.first })
end
# vuln-code-snippet end ruby_sqli_mysql2_param
