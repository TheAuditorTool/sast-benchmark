require_relative 'shared'

# vuln-code-snippet start ruby_sqli_integer_cast
def get_order_details(req)
  db = get_db_connection
  id = req.param('id').to_i
  return BenchmarkResponse.bad_request('invalid id') if id <= 0
  rows = db.execute("SELECT id, total, status, created_at FROM orders WHERE id = #{id}")  # vuln-code-snippet safe-line ruby_sqli_integer_cast
  return BenchmarkResponse.bad_request('order not found') if rows.empty?
  order = rows.first
  BenchmarkResponse.json({ id: order[0], total: order[1], status: order[2], created_at: order[3] })
end
# vuln-code-snippet end ruby_sqli_integer_cast
