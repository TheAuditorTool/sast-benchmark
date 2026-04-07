require_relative 'shared'

VARY_OPTS = %w[Accept Accept-Encoding Accept-Language].freeze

# vuln-code-snippet start ruby_headerinj_allowlist_vary
def set_vary_allowlisted(req)
  h = req.param('vary')
  vary_value = VARY_OPTS.include?(h) ? h : 'Accept' # vuln-code-snippet safe-line ruby_headerinj_allowlist_vary
  response = BenchmarkResponse.ok('ok')
  response.headers['Vary'] = vary_value
  response
end
# vuln-code-snippet end ruby_headerinj_allowlist_vary
