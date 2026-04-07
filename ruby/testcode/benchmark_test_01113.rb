require_relative 'shared'

class CsvExporter; def export; 'csv'; end; end
class PdfExporter; def export; 'pdf'; end; end

SAFE_CLASSES = { 'csv' => CsvExporter, 'pdf' => PdfExporter }.freeze

def create_handler_safe(req)
  type = req.param('type')
  klass = SAFE_CLASSES[type]
  return BenchmarkResponse.bad_request('unknown') unless klass
  BenchmarkResponse.ok(klass.new.export)
end
