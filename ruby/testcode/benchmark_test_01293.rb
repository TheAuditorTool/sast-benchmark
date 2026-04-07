require_relative 'shared'

def list_orders(req)
  db = get_db_connection
  status = req.param('status')
  sort = req.param('sort')
  rows = db.execute("SELECT id, customer, total FROM orders WHERE status = '#{status}' ORDER BY #{sort}")
  BenchmarkResponse.json({ orders: rows.map { |r| { id: r[0], customer: r[1], total: r[2] } } })
end
