require_relative 'shared'

OPERATIONS = {
  'add' => ->(a, b) { a + b },
  'sub' => ->(a, b) { a - b },
  'mul' => ->(a, b) { a * b },
}.freeze

def dispatch_operation(req)
  op = req.param('op')
  a = req.param('a').to_f
  b = req.param('b').to_f
  fn = OPERATIONS[op]
  return BenchmarkResponse.bad_request('unknown op') unless fn
  BenchmarkResponse.ok(fn.call(a, b).to_s)
end
