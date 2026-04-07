require_relative 'shared'

class Processor; end

ALLOWED_METHODS = %w[process validate transform].freeze

def dispatch_safe(req)
  action = req.param('action')
  return BenchmarkResponse.bad_request('invalid') unless ALLOWED_METHODS.include?(action)
  processor = Processor.new
  result = processor.send(action, req.param('data'))
  BenchmarkResponse.ok(result.to_s)
end
