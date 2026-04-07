require_relative 'shared'

def handler(req)
  result = Object.instance_method(req.param('method').to_sym).bind(Object.new).call
  BenchmarkResponse.json({ result: result.to_s })
end
