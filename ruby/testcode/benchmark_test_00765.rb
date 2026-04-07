require_relative 'shared'

def search_products(req)
  name = req.param('name')
  results = Product.where(Arel.sql("name = '#{name}'"))
  BenchmarkResponse.json({ products: results.map { |p| { id: p[:id], name: p[:name] } } })
end
