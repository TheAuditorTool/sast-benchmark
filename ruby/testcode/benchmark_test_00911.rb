require_relative 'shared'

def orders_above_threshold(req)
  threshold = req.param('threshold').to_i
  orders = Order.group(:user_id).having("count(*) > ?", threshold)
  BenchmarkResponse.json({ groups: orders.length })
end
