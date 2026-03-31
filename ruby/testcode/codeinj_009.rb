require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_module_eval
def add_module_method(req)
  code = req.param('code')
  Kernel.module_eval(code) # vuln-code-snippet vuln-line ruby_codeinj_module_eval
  BenchmarkResponse.ok("module updated")
end
# vuln-code-snippet end ruby_codeinj_module_eval
