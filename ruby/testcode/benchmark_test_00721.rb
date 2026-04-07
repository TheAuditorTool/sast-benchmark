require_relative 'shared'

def load_plugin(req)
  plugin_name = req.param('plugin')
  load("plugins/#{plugin_name}.rb")
  BenchmarkResponse.ok("plugin loaded")
end
