require_relative 'shared'

# vuln-code-snippet start ruby_sqli_having_safe
def orders_above_threshold(req)
  threshold = req.param('threshold').to_i
  orders = Order.group(:user_id).having("count(*) > ?", threshold)  # vuln-code-snippet safe-line ruby_sqli_having_safe
  BenchmarkResponse.json({ groups: orders.length })
end
# vuln-code-snippet end ruby_sqli_having_safe
