require_relative 'shared'

class DataService
  def summary(data) = "summary:#{data}"
  def totals(data)  = "totals:#{data}"
end

ALLOWED_ACTIONS = %w[summary totals].freeze

# vuln-code-snippet start ruby_reflect_send_allowlist
def reflect_send_allowlist(req)
  method_name = req.param('action')
  return BenchmarkResponse.bad_request('forbidden') unless ALLOWED_ACTIONS.include?(method_name)
  result = DataService.new.send(method_name, req.param('data')) # vuln-code-snippet safe-line ruby_reflect_send_allowlist
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_reflect_send_allowlist
