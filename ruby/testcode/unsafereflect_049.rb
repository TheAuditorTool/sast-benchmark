require_relative 'shared'

SAFE_NAMES = %w[UserReport AdminReport].freeze

# vuln-code-snippet start ruby_reflect_namespace_allowlist
def handler(req)
  n = req.param('name')
  raise ArgumentError, 'name not allowed' unless SAFE_NAMES.include?(n) # vuln-code-snippet safe-line ruby_reflect_namespace_allowlist
  klass = Object.const_get("Reports::#{n}")
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_reflect_namespace_allowlist
