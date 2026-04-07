require_relative 'shared'

class DataService
  def summary(data) = "summary:#{data}"
  def totals(data)  = "totals:#{data}"
end

ALLOWED_ACTIONS = %w[summary totals].freeze

def reflect_send_allowlist(req)
  method_name = req.param('action')
  return BenchmarkResponse.bad_request('forbidden') unless ALLOWED_ACTIONS.include?(method_name)
  result = DataService.new.send(method_name, req.param('data'))
  BenchmarkResponse.ok(result.to_s)
end
