require_relative 'shared'

# vuln-code-snippet start ruby_fi_instance_eval_file
def handler(req)
  instance_eval(File.read(req.param('path'))) # vuln-code-snippet vuln-line ruby_fi_instance_eval_file
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_instance_eval_file
