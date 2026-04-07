require_relative 'shared'

def handler(req)
  obj = Object.new
  result = obj.method(req.param('action').to_sym).call(req.param('arg'))
  BenchmarkResponse.json({ result: result.to_s })
end
