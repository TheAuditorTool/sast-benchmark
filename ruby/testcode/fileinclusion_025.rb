require_relative 'shared'

# vuln-code-snippet start ruby_fi_eval_file_read
def handler(req)
  eval(File.read("templates/#{req.param('name')}.rb")) # vuln-code-snippet vuln-line ruby_fi_eval_file_read
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_eval_file_read
