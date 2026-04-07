require_relative 'shared'

SAFE = %w[String Integer Float].freeze

# vuln-code-snippet start ruby_reflect_allowlist_const
def handler(req)
  n = req.param('klass')
  raise ArgumentError, 'not allowed' unless SAFE.include?(n) # vuln-code-snippet safe-line ruby_reflect_allowlist_const
  klass = Object.const_get(n)
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
# vuln-code-snippet end ruby_reflect_allowlist_const
