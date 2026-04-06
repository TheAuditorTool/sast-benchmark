require_relative 'shared'

class CsvExporter; def export; 'csv'; end; end
class PdfExporter; def export; 'pdf'; end; end

SAFE_CLASSES = { 'csv' => CsvExporter, 'pdf' => PdfExporter }.freeze

# vuln-code-snippet start ruby_reflect_safe_const_checked
def create_handler_safe(req)
  type = req.param('type')
  klass = SAFE_CLASSES[type]
  return BenchmarkResponse.bad_request('unknown') unless klass # vuln-code-snippet safe-line ruby_reflect_safe_const_checked
  BenchmarkResponse.ok(klass.new.export)
end
# vuln-code-snippet end ruby_reflect_safe_const_checked
