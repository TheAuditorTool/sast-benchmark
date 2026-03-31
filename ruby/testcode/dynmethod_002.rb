require_relative 'shared'

ALLOWED_DEFS = %w[greet farewell].freeze

class GreeterService
end

ALLOWED_DEFS.each do |name|
  GreeterService.define_method(name) { |who| "#{name}: #{who}" }
end

# vuln-code-snippet start ruby_dynmethod_define_allowlist
def dynmethod_define_allowlist(req)
  method_name = req.param('action')
  return BenchmarkResponse.bad_request('forbidden') unless ALLOWED_DEFS.include?(method_name)
  result = GreeterService.new.send(method_name, req.param('name')) # vuln-code-snippet safe-line ruby_dynmethod_define_allowlist
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_dynmethod_define_allowlist
