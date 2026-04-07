require_relative 'shared'

TRUSTED_PROXIES = %w[10.0.0.1 10.0.0.2].freeze

def set_forwarded_host_validated(req)
  fwd = req.header('HTTP_X_FORWARDED_HOST')
  remote_addr = req.header('REMOTE_ADDR')
  safe_host = TRUSTED_PROXIES.include?(remote_addr) ? fwd : 'unknown'
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Host'] = safe_host
  response
end
