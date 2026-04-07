require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_basicobj_eval
def evaluate_expr(req)
  expr = req.param('expr')
  result = BasicObject.instance_eval(expr) # vuln-code-snippet vuln-line ruby_codeinj_basicobj_eval
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_basicobj_eval
