require_relative 'shared'

def handler(req)
  result = eval("method(:#{req.param('method_name')})").call
  BenchmarkResponse.json({ result: result.to_s })
end
