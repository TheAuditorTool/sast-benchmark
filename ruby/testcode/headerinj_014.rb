require_relative 'shared'

VALID_TYPES = %w[text/html application/json text/plain].freeze

# vuln-code-snippet start ruby_headerinj_allowlist_val
def content_type_safe(req)
  ctype = req.param('type')
  return BenchmarkResponse.bad_request('invalid') unless VALID_TYPES.include?(ctype) # vuln-code-snippet safe-line ruby_headerinj_allowlist_val
  BenchmarkResponse.new(200, 'ok', { 'Content-Type' => ctype })
end
# vuln-code-snippet end ruby_headerinj_allowlist_val
