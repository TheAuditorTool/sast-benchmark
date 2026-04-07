require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_proc_eval
def run_expression(req)
  proc_body = req.param('expr')
  result = Proc.new { eval(proc_body) }.call # vuln-code-snippet vuln-line ruby_codeinj_proc_eval
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_proc_eval
