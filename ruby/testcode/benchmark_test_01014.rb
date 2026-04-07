require_relative 'shared'

ALLOWED_PLUGINS = %w[cache logger mailer].freeze

def load_allowed_plugin(req)
  name = req.param('plugin')
  return BenchmarkResponse.bad_request('unknown') unless ALLOWED_PLUGINS.include?(name)
  load("plugins/#{name}.rb")
  BenchmarkResponse.ok("loaded #{name}")
end
