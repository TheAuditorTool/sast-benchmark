require_relative 'shared'

CORS_ALLOWED_ORIGINS = %w[https://app.example.com https://admin.example.com].freeze

# vuln-code-snippet start ruby_headerinj_allowlist_origin
def set_cors_allowlisted(req)
  origin = req.param('origin')
  safe_origin = CORS_ALLOWED_ORIGINS.include?(origin) ? origin : 'null' # vuln-code-snippet safe-line ruby_headerinj_allowlist_origin
  response = BenchmarkResponse.ok('ok')
  response.headers['Access-Control-Allow-Origin'] = safe_origin
  response
end
# vuln-code-snippet end ruby_headerinj_allowlist_origin
