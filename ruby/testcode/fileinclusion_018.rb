require_relative 'shared'

PLUGIN_MAP = {
  'cache' => 'plugins/cache_plugin.rb',
  'logger' => 'plugins/logger_plugin.rb',
  'mailer' => 'plugins/mailer_plugin.rb',
}.freeze

# vuln-code-snippet start ruby_fi_allowlist_map
def load_mapped_plugin(req)
  name = req.param('plugin')
  path = PLUGIN_MAP[name] # vuln-code-snippet safe-line ruby_fi_allowlist_map
  return BenchmarkResponse.bad_request('unknown plugin') unless path
  load(path)
  BenchmarkResponse.ok("loaded #{name}")
end
# vuln-code-snippet end ruby_fi_allowlist_map
