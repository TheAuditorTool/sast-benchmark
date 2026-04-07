require_relative 'shared'

class PdfHandler
  def process(data) = "pdf:#{data}"
end

class CsvHandler
  def process(data) = "csv:#{data}"
end

HANDLER_MAP = { 'pdf' => PdfHandler, 'csv' => CsvHandler }.freeze

def reflect_map_lookup(req)
  type = req.param('type')
  handler_class = HANDLER_MAP[type]
  return BenchmarkResponse.bad_request('unknown type') unless handler_class
  BenchmarkResponse.ok(handler_class.new.process(req.param('data')))
end
