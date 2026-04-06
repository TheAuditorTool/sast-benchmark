require_relative 'shared'

class CsvExporter; def export; 'csv'; end; end
class PdfExporter; def export; 'pdf'; end; end

FACTORIES = { 'csv' => CsvExporter, 'pdf' => PdfExporter }.freeze

# vuln-code-snippet start ruby_reflect_factory_map
def export_factory(req)
  type = req.param('type')
  klass = FACTORIES[type] # vuln-code-snippet safe-line ruby_reflect_factory_map
  return BenchmarkResponse.bad_request('unknown type') unless klass
  BenchmarkResponse.ok(klass.new.export)
end
# vuln-code-snippet end ruby_reflect_factory_map
