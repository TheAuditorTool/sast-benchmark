require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_binding_expr
def eval_in_binding(req)
  template = req.param('expr')
  result = binding.eval(template) # vuln-code-snippet vuln-line ruby_codeinj_binding_expr
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_binding_expr
