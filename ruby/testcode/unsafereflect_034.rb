require_relative 'shared'

# vuln-code-snippet start ruby_reflect_safe_const_bypass
def handler(req)
  # gsub stripping non-alphanumeric chars is insufficient — namespaced class names with :: pass through
  safe = req.param('klass').gsub(/[^A-Za-z0-9:]/, '')
  klass = Object.const_get(safe) # vuln-code-snippet vuln-line ruby_reflect_safe_const_bypass
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
# vuln-code-snippet end ruby_reflect_safe_const_bypass
