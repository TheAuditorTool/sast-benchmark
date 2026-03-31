require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_send_eval
def run_script(req)
  user_input = req.param('script')
  result = Kernel.send(:eval, user_input) # vuln-code-snippet vuln-line ruby_codeinj_send_eval
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_send_eval
