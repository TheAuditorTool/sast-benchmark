require_relative 'shared'

TRUSTED_PROXIES = %w[10.0.0.1 10.0.0.2].freeze

# vuln-code-snippet start ruby_headerinj_forwarded_validated
def set_forwarded_host_validated(req)
  fwd = req.header('HTTP_X_FORWARDED_HOST')
  remote_addr = req.header('REMOTE_ADDR')
  safe_host = TRUSTED_PROXIES.include?(remote_addr) ? fwd : 'unknown' # vuln-code-snippet safe-line ruby_headerinj_forwarded_validated
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Host'] = safe_host
  response
end
# vuln-code-snippet end ruby_headerinj_forwarded_validated
