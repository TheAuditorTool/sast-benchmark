require_relative 'shared'

# vuln-code-snippet start ruby_fi_open_and_eval
def handler(req)
  open(req.param('file')) { |f| eval(f.read) } # vuln-code-snippet vuln-line ruby_fi_open_and_eval
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_open_and_eval
