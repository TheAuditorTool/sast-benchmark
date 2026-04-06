require_relative 'shared'

OPERATIONS = {
  'add' => ->(a, b) { a + b },
  'sub' => ->(a, b) { a - b },
  'mul' => ->(a, b) { a * b },
}.freeze

# vuln-code-snippet start ruby_codeinj_method_lookup
def dispatch_operation(req)
  op = req.param('op')
  a = req.param('a').to_f
  b = req.param('b').to_f
  fn = OPERATIONS[op] # vuln-code-snippet safe-line ruby_codeinj_method_lookup
  return BenchmarkResponse.bad_request('unknown op') unless fn
  BenchmarkResponse.ok(fn.call(a, b).to_s)
end
# vuln-code-snippet end ruby_codeinj_method_lookup
