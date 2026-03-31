require_relative 'shared'

ALLOWED_PLUGINS = %w[csv_importer xml_exporter pdf_renderer image_resizer].freeze

# vuln-code-snippet start ruby_fi_load_allowlist
def load_approved_plugin(req)
  plugin_name = req.param('plugin')
  unless ALLOWED_PLUGINS.include?(plugin_name) # vuln-code-snippet safe-line ruby_fi_load_allowlist
    return BenchmarkResponse.bad_request("unknown plugin")
  end
  load("plugins/#{plugin_name}.rb")
  BenchmarkResponse.ok("plugin loaded")
end
# vuln-code-snippet end ruby_fi_load_allowlist
