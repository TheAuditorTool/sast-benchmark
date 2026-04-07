require_relative 'shared'

def add_integers(req)
  a = Integer(req.param('a'))
  b = Integer(req.param('b'))
  result = a + b
  BenchmarkResponse.json({ result: result })
end
