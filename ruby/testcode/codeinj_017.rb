require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_basic_eval
def eval_basic(req)
  expr = req.param('expr')
  result = BasicObject.new.instance_eval(expr) # vuln-code-snippet vuln-line ruby_codeinj_basic_eval
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_basic_eval
