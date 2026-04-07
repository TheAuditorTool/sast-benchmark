require_relative 'shared'

class Order
  def self.where(conditions)
    [{ id: 1, status: conditions[:status], tenant_id: 'tenant_A' },
     { id: 2, status: conditions[:status], tenant_id: 'tenant_B' }]
  end
end

# vuln-code-snippet start ruby_authz_multitenant_query
def list_orders_by_status(req)
  status = req.param('status')
  orders = Order.where(status: status) # vuln-code-snippet vuln-line ruby_authz_multitenant_query
  BenchmarkResponse.json(orders)
end
# vuln-code-snippet end ruby_authz_multitenant_query
