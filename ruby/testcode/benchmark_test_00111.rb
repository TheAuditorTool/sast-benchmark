require_relative 'shared'

def load_safe_plugin(req)
  name = req.param('plugin')
  unless name.match?(/\A[a-z_]+\z/) && !name.include?('..')
    return BenchmarkResponse.bad_request("invalid plugin name")
  end
  load("plugins/#{name}.rb")
  BenchmarkResponse.ok("plugin loaded")
end
