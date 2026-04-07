require_relative 'shared'

# vuln-code-snippet start ruby_fi_require_constant_only
def handler(req)
  require 'json' # vuln-code-snippet safe-line ruby_fi_require_constant_only
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_require_constant_only
