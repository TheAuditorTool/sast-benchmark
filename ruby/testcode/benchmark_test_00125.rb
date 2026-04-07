require_relative 'shared'

def then_dispatch(req)
  transform = req.param('transform')
  result = 42.then { |v| v.send(transform) }
  BenchmarkResponse.json({ result: result.to_s })
end
