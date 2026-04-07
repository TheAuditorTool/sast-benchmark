require_relative 'shared'

ALLOWED = { 'pdf' => 'PdfReport', 'csv' => 'CsvReport' }.freeze

# vuln-code-snippet start ruby_reflect_zeitwerk_allowlist
def handler(req)
  name = ALLOWED.fetch(req.param('key')) # vuln-code-snippet safe-line ruby_reflect_zeitwerk_allowlist
  klass = Object.const_get("Reports::#{name}")
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_reflect_zeitwerk_allowlist
