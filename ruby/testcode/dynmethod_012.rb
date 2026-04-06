require_relative 'shared'

class Formatter
  def self.csv(data); "csv:#{data}"; end
  def self.json(data); "json:#{data}"; end
  def self.xml(data); "xml:#{data}"; end
end

# vuln-code-snippet start ruby_dynmethod_public_send_fixed
def format_data(req)
  data = req.param('data')
  result = Formatter.public_send(:json, data) # vuln-code-snippet safe-line ruby_dynmethod_public_send_fixed
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_dynmethod_public_send_fixed
