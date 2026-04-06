require_relative 'shared'

class Processor; end

ALLOWED_METHODS = %w[process validate transform].freeze

# vuln-code-snippet start ruby_dynmethod_send_allowlist
def dispatch_safe(req)
  action = req.param('action')
  return BenchmarkResponse.bad_request('invalid') unless ALLOWED_METHODS.include?(action) # vuln-code-snippet safe-line ruby_dynmethod_send_allowlist
  processor = Processor.new
  result = processor.send(action, req.param('data'))
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_dynmethod_send_allowlist
