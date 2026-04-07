require_relative 'shared'

# vuln-code-snippet start ruby_sqli_having_inject
def orders_above_count(req)
  threshold = req.param('threshold')
  orders = Order.group(:user_id).having("count(*) > #{threshold}")  # vuln-code-snippet vuln-line ruby_sqli_having_inject
  BenchmarkResponse.json({ order_groups: orders.length })
end
# vuln-code-snippet end ruby_sqli_having_inject
