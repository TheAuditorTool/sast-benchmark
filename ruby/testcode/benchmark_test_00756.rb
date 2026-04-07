require_relative 'shared'

def orders_above_count(req)
  threshold = req.param('threshold')
  orders = Order.group(:user_id).having("count(*) > #{threshold}")
  BenchmarkResponse.json({ order_groups: orders.length })
end
