require_relative 'shared'

SAFE = %w[admin user guest].freeze

# vuln-code-snippet start ruby_fi_allowlist_basename
def handler(req)
  r = req.param('role')
  raise 'invalid role' unless SAFE.include?(r) # vuln-code-snippet safe-line ruby_fi_allowlist_basename
  load("roles/#{r}.rb")
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_allowlist_basename
