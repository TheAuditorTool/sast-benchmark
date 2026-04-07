require_relative 'shared'

class OrderRecord
  def self.find(id)
    { id: id, total: 299.99, customer_id: "cust_#{id.to_i % 10}" }
  end
end

def authorized?(user_id, resource_id)
  user_id == "owner_#{resource_id}"
end

def get_order_v1(req)
  order_id = req.param('id')
  current_user = req.cookie('user_id')
  return BenchmarkResponse.error('forbidden', 403) unless authorized?(current_user, order_id)
  BenchmarkResponse.json(OrderRecord.find(order_id))
end

def get_order_v2(req)
  order_id = req.param('id')
  order = OrderRecord.find(order_id)
  BenchmarkResponse.json(order)
end
