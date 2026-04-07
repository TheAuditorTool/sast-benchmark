require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_module_exec
def extend_module(req)
  tainted_code = req.param('code')
  result = Module.new.module_exec { eval(tainted_code) } # vuln-code-snippet vuln-line ruby_codeinj_module_exec
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_module_exec
