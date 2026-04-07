require_relative 'shared'

PLUGIN_MAP = {
  'cache' => 'plugins/cache_plugin.rb',
  'logger' => 'plugins/logger_plugin.rb',
  'mailer' => 'plugins/mailer_plugin.rb',
}.freeze

def load_mapped_plugin(req)
  name = req.param('plugin')
  path = PLUGIN_MAP[name]
  return BenchmarkResponse.bad_request('unknown plugin') unless path
  load(path)
  BenchmarkResponse.ok("loaded #{name}")
end
