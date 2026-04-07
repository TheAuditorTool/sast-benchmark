require_relative 'shared'

# vuln-code-snippet start ruby_fi_autoload_user_sym
def handler(req)
  autoload(req.param('class_name').to_sym, req.param('path')) # vuln-code-snippet vuln-line ruby_fi_autoload_user_sym
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_autoload_user_sym
