require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_instance_eval
def configure_object(req)
  expr = req.param('expr')
  obj = Object.new
  obj.instance_eval(expr) # vuln-code-snippet vuln-line ruby_codeinj_instance_eval
  BenchmarkResponse.ok("configured")
end
# vuln-code-snippet end ruby_codeinj_instance_eval
