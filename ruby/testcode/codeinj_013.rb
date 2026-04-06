require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_kernel_eval
def eval_body(req)
  code = req.body_str
  result = Kernel.eval(code) # vuln-code-snippet vuln-line ruby_codeinj_kernel_eval
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_kernel_eval
