require_relative 'shared'

# vuln-code-snippet start ruby_csrf_api_no_origin
def api_create_order(req)
  product_id = req.param('product_id').to_i
  quantity = req.param('quantity').to_i
  # No Origin check, no CSRF token — any origin can POST
  db = get_db_connection
  db.execute("INSERT INTO orders (product_id, quantity) VALUES (?, ?)", [product_id, quantity])  # vuln-code-snippet vuln-line ruby_csrf_api_no_origin
  BenchmarkResponse.json({ result: 'order_created' })
end
# vuln-code-snippet end ruby_csrf_api_no_origin
