require_relative 'shared'

def transform_value(req)
  transform = req.param('transform')
  value = 42
  result = value.then { eval(transform) }
  BenchmarkResponse.json({ result: result.to_s })
end
