require_relative 'shared'

SAFE_OPS = { 'add' => :+, 'sub' => :-, 'mul' => :*, 'div' => :/ }.freeze

def safe_arithmetic(req)
  op = SAFE_OPS.fetch(req.param('op')) { return BenchmarkResponse.bad_request('unknown op') }
  left = Integer(req.param('left'))
  right = Integer(req.param('right'))
  result = left.send(op, right)
  BenchmarkResponse.json({ result: result })
end
