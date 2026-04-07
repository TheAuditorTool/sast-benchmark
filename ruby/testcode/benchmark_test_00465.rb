require_relative 'shared'

class Order
  def self.where(conditions)
    [{ id: 1, status: conditions[:status], tenant_id: 'tenant_A' },
     { id: 2, status: conditions[:status], tenant_id: 'tenant_B' }]
  end
end

def list_orders_by_status(req)
  status = req.param('status')
  orders = Order.where(status: status)
  BenchmarkResponse.json(orders)
end
