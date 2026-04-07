require_relative 'shared'

# vuln-code-snippet start ruby_fi_binding_eval_file2
def handler(req)
  binding.eval(IO.read("scripts/#{req.param('script')}.rb")) # vuln-code-snippet vuln-line ruby_fi_binding_eval_file2
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_binding_eval_file2
