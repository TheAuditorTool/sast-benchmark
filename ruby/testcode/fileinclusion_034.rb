require_relative 'shared'

# vuln-code-snippet start ruby_fi_autoload_remote
def handler(req)
  autoload(:Plugin, "http://#{req.param('host')}/plugin.rb") # vuln-code-snippet vuln-line ruby_fi_autoload_remote
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_autoload_remote
