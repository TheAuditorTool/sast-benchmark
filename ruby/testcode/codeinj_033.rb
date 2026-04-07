require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_template_eval
def dispatch_method(req)
  method_name = req.param('method_name')
  ctx = binding
  result = ctx.eval("self.#{method_name}") # vuln-code-snippet vuln-line ruby_codeinj_template_eval
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_template_eval
