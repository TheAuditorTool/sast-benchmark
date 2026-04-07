require_relative 'shared'

PLUGINS = { 'pdf' => 'plugins/pdf.rb', 'csv' => 'plugins/csv.rb' }.freeze

# vuln-code-snippet start ruby_fi_plugin_registry
def handler(req)
  load(PLUGINS.fetch(req.param('key'))) # vuln-code-snippet safe-line ruby_fi_plugin_registry
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_plugin_registry
