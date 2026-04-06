require_relative 'shared'

ALLOWED_PLUGINS = %w[cache logger mailer].freeze

# vuln-code-snippet start ruby_fi_require_frozen
def load_allowed_plugin(req)
  name = req.param('plugin')
  return BenchmarkResponse.bad_request('unknown') unless ALLOWED_PLUGINS.include?(name) # vuln-code-snippet safe-line ruby_fi_require_frozen
  load("plugins/#{name}.rb")
  BenchmarkResponse.ok("loaded #{name}")
end
# vuln-code-snippet end ruby_fi_require_frozen
