require_relative 'shared'

ALLOWED = { 'pdf' => 'PdfReport', 'csv' => 'CsvReport' }.freeze

def handler(req)
  name = ALLOWED.fetch(req.param('key'))
  klass = Object.const_get("Reports::#{name}")
  BenchmarkResponse.json({ ok: true })
end
