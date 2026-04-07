require_relative 'shared'

def register_adapter(req)
  name = req.param('name')
  autoload(name.to_sym, "adapters/#{name}")
  BenchmarkResponse.ok("adapter registered")
end
