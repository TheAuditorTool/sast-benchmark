require_relative 'shared'

class CsvExporter; def export; 'csv'; end; end
class PdfExporter; def export; 'pdf'; end; end
class XlsExporter; def export; 'xls'; end; end

REGISTRY = { 'csv' => CsvExporter, 'pdf' => PdfExporter, 'xls' => XlsExporter }.freeze

# vuln-code-snippet start ruby_reflect_frozen_registry
def export_registered(req)
  type = req.param('type')
  klass = REGISTRY.fetch(type) { return BenchmarkResponse.bad_request('unknown') } # vuln-code-snippet safe-line ruby_reflect_frozen_registry
  BenchmarkResponse.ok(klass.new.export)
end
# vuln-code-snippet end ruby_reflect_frozen_registry
