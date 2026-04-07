require_relative 'shared'

# vuln-code-snippet start ruby_fi_autoload_fixed_sym
def handler(req)
  autoload(:User, 'models/user') # vuln-code-snippet safe-line ruby_fi_autoload_fixed_sym
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_autoload_fixed_sym
