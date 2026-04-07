require_relative 'shared'

# vuln-code-snippet start ruby_sqli_arel_node_raw
def search_products(req)
  name = req.param('name')
  results = Product.where(Arel.sql("name = '#{name}'"))  # vuln-code-snippet vuln-line ruby_sqli_arel_node_raw
  BenchmarkResponse.json({ products: results.map { |p| { id: p[:id], name: p[:name] } } })
end
# vuln-code-snippet end ruby_sqli_arel_node_raw
