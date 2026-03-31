require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_eval
def execute_code(req)
  code = req.param('code')
  result = eval(code) # vuln-code-snippet vuln-line ruby_codeinj_eval
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_eval
