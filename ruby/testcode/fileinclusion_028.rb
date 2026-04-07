require_relative 'shared'

# vuln-code-snippet start ruby_fi_require_relative_taint2
def handler(req)
  require_relative "../modules/#{req.param('mod')}" # vuln-code-snippet vuln-line ruby_fi_require_relative_taint2
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_require_relative_taint2
