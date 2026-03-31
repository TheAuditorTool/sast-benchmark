require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_binding_eval
def eval_expression(req)
  expression = req.param('expression')
  x = 42
  result = binding.eval(expression) # vuln-code-snippet vuln-line ruby_codeinj_binding_eval
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_binding_eval
