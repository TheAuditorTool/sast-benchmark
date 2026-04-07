require_relative 'shared'

class CsvExporter; def export; 'csv'; end; end
class PdfExporter; def export; 'pdf'; end; end

FACTORIES = { 'csv' => CsvExporter, 'pdf' => PdfExporter }.freeze

def export_factory(req)
  type = req.param('type')
  klass = FACTORIES[type]
  return BenchmarkResponse.bad_request('unknown type') unless klass
  BenchmarkResponse.ok(klass.new.export)
end
