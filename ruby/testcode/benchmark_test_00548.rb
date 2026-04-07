require_relative 'shared'

def api_create_order(req)
  product_id = req.param('product_id').to_i
  quantity = req.param('quantity').to_i
  db = get_db_connection
  db.execute("INSERT INTO orders (product_id, quantity) VALUES (?, ?)", [product_id, quantity])
  BenchmarkResponse.json({ result: 'order_created' })
end
