require_relative 'shared'

class FormatterService
  def csv(data)  = "csv:#{data}"
  def json(data) = "json:#{data}"
end

SAFE_METHODS = %w[csv json].freeze

# vuln-code-snippet start ruby_reflect_respond_to
def reflect_respond_to(req)
  method = req.param('format')
  obj = FormatterService.new
  if obj.respond_to?(method) && SAFE_METHODS.include?(method) # vuln-code-snippet safe-line ruby_reflect_respond_to
    BenchmarkResponse.ok(obj.send(method, req.param('data')))
  else
    BenchmarkResponse.bad_request('unsupported format')
  end
end
# vuln-code-snippet end ruby_reflect_respond_to
