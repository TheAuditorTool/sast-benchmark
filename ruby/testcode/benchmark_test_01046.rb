require_relative 'shared'

def calculate(req)
  a = Integer(req.param('a'))
  b = Integer(req.param('b'))
  op = req.param('op')
  result = case op
           when 'add' then a + b
           when 'sub' then a - b
           when 'mul' then a * b
           else 0
           end
  BenchmarkResponse.ok(result.to_s)
end
