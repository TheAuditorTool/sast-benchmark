require_relative 'shared'

# vuln-code-snippet start ruby_fi_load_erb_template
def handler(req)
  load("views/#{req.param('view')}.rb") # vuln-code-snippet vuln-line ruby_fi_load_erb_template
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_load_erb_template
