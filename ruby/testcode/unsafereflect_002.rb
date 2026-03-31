require_relative 'shared'

class PdfHandler
  def process(data) = "pdf:#{data}"
end

class CsvHandler
  def process(data) = "csv:#{data}"
end

HANDLER_MAP = { 'pdf' => PdfHandler, 'csv' => CsvHandler }.freeze

# vuln-code-snippet start ruby_reflect_map_lookup
def reflect_map_lookup(req)
  type = req.param('type')
  handler_class = HANDLER_MAP[type] # vuln-code-snippet safe-line ruby_reflect_map_lookup
  return BenchmarkResponse.bad_request('unknown type') unless handler_class
  BenchmarkResponse.ok(handler_class.new.process(req.param('data')))
end
# vuln-code-snippet end ruby_reflect_map_lookup
