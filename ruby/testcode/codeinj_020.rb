require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_sandbox_int
def sandbox_eval(req)
  expr = req.param('expr')
  return BenchmarkResponse.bad_request('only numbers') unless expr.match?(/\A[\d+\-*\/() .]+\z/) # vuln-code-snippet safe-line ruby_codeinj_sandbox_int
  result = eval(expr)
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_sandbox_int
