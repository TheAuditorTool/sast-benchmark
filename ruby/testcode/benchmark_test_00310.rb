require_relative 'shared'

def const_dispatch(req)
  klass  = Object.const_get(req.param('klass'))
  result = klass.send(req.param('method'))
  BenchmarkResponse.json({ result: result.to_s })
end
