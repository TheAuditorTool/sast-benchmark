require_relative 'shared'

# vuln-code-snippet start ruby_sqli_pg_exec
def get_customer_orders(req)
  conn = get_db_connection
  customer_id = req.param('customer_id')
  page = req.param('page', default: '1').to_i
  limit = 50
  offset = (page - 1) * limit
  result = conn.exec("SELECT * FROM orders WHERE customer_id = #{customer_id} ORDER BY created_at DESC LIMIT #{limit} OFFSET #{offset}")  # vuln-code-snippet vuln-line ruby_sqli_pg_exec
  orders = result.map { |row| { id: row['id'], total: row['total'], status: row['status'] } }
  BenchmarkResponse.json({ orders: orders, page: page })
end
# vuln-code-snippet end ruby_sqli_pg_exec
