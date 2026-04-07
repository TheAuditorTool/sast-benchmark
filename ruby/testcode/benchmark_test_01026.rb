require_relative 'shared'

def list_products_sorted(req)
  column    = req.param('sort', default: 'name')
  direction = req.param('dir',  default: 'ASC')
  results   = FakeActiveRecord::Base.where("1=1").order("#{column} #{direction}")
  BenchmarkResponse.json({ products: results.to_a, sort: column, direction: direction })
end
