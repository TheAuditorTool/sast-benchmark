require_relative 'shared'

ALLOWED_DEFS = %w[greet farewell].freeze

class GreeterService
end

ALLOWED_DEFS.each do |name|
  GreeterService.define_method(name) { |who| "#{name}: #{who}" }
end

def handler(req)
  method_name = req.param('action')
  return BenchmarkResponse.bad_request('forbidden') unless ALLOWED_DEFS.include?(method_name)
  result = GreeterService.new.send(method_name, req.param('name'))
  BenchmarkResponse.ok(result)
end
