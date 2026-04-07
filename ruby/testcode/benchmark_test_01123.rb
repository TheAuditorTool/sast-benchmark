require_relative 'shared'

class Formatter
  def self.csv(data); "csv:#{data}"; end
  def self.json(data); "json:#{data}"; end
  def self.xml(data); "xml:#{data}"; end
end

def format_data(req)
  data = req.param('data')
  result = Formatter.public_send(:json, data)
  BenchmarkResponse.ok(result)
end
