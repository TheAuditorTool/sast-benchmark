require_relative 'shared'

def require_gem(req)
  gem_name = req.param('gem')
  require gem_name
  BenchmarkResponse.ok("loaded #{gem_name}")
end
