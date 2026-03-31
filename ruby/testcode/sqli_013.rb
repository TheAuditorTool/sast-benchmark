require_relative 'shared'

# vuln-code-snippet start ruby_sqli_order_interp
def list_products_sorted(req)
  column    = req.param('sort', default: 'name')
  direction = req.param('dir',  default: 'ASC')
  results   = FakeActiveRecord::Base.where("1=1").order("#{column} #{direction}")  # vuln-code-snippet vuln-line ruby_sqli_order_interp
  BenchmarkResponse.json({ products: results.to_a, sort: column, direction: direction })
end
# vuln-code-snippet end ruby_sqli_order_interp
