require_relative 'shared'

ALLOWED_PLUGINS = %w[csv_importer xml_exporter pdf_renderer image_resizer].freeze

def load_approved_plugin(req)
  plugin_name = req.param('plugin')
  unless ALLOWED_PLUGINS.include?(plugin_name)
    return BenchmarkResponse.bad_request("unknown plugin")
  end
  load("plugins/#{plugin_name}.rb")
  BenchmarkResponse.ok("plugin loaded")
end
