require_relative 'shared'

def list_orders(req)
  db = get_db_connection
  status = req.param('status')
  allowed_sorts = ['created_at', 'total', 'customer']
  sort = allowed_sorts.include?(req.param('sort')) ? req.param('sort') : 'created_at'
  rows = db.execute("SELECT id, customer, total FROM orders WHERE status = ?", [status])
  BenchmarkResponse.json({ orders: rows.map { |r| { id: r[0], customer: r[1], total: r[2] } } })
end
