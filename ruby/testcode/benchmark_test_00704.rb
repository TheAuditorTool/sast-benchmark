require_relative 'shared'

PLUGINS = { 'pdf' => 'plugins/pdf.rb', 'csv' => 'plugins/csv.rb' }.freeze

def handler(req)
  load(PLUGINS.fetch(req.param('key')))
  BenchmarkResponse.json({ ok: true })
end
