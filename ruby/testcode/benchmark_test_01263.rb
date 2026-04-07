require_relative 'shared'

ALLOWED_METHODS = %w[upcase downcase reverse length].freeze

def transform_string(req)
  method_name = req.param('method')
  input = req.param('input')
  unless ALLOWED_METHODS.include?(method_name)
    return BenchmarkResponse.bad_request("unknown method")
  end
  result = input.public_send(method_name)
  BenchmarkResponse.ok(result.to_s)
end
