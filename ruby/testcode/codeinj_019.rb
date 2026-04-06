require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_thread_eval
def eval_in_thread(req)
  code = req.param('code')
  result = nil
  t = Thread.new { result = eval(code) } # vuln-code-snippet vuln-line ruby_codeinj_thread_eval
  t.join(5)
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_thread_eval
