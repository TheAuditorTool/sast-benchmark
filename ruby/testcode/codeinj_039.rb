require_relative 'shared'

SAFE_OPS = { 'add' => :+, 'sub' => :-, 'mul' => :*, 'div' => :/ }.freeze

# vuln-code-snippet start ruby_codeinj_allowlist_method_call
def safe_arithmetic(req)
  op = SAFE_OPS.fetch(req.param('op')) { return BenchmarkResponse.bad_request('unknown op') }
  left = Integer(req.param('left'))
  right = Integer(req.param('right'))
  result = left.send(op, right) # vuln-code-snippet safe-line ruby_codeinj_allowlist_method_call
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_codeinj_allowlist_method_call
