require_relative 'shared'

# vuln-code-snippet start ruby_reflect_autoload_trigger2
def handler(req)
  autoload(req.param('const').to_sym, req.param('path')) # vuln-code-snippet vuln-line ruby_reflect_autoload_trigger2
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_reflect_autoload_trigger2
