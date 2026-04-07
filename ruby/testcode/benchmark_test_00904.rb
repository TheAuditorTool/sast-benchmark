require_relative 'shared'

def alias_dynamic(req)
  klass = Class.new
  klass.send(:alias_method, req.param('new_name').to_sym, :to_s)
  BenchmarkResponse.json({ result: "aliased" })
end
