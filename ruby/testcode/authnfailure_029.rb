require_relative 'shared'

TRUSTED_IP = '10.0.0.1'.freeze

# vuln-code-snippet start ruby_authn_ip_only
def authenticate_by_ip(req)
  remote_addr = req.header('REMOTE_ADDR')
  return BenchmarkResponse.error('Unauthorized', 401) unless remote_addr == TRUSTED_IP # vuln-code-snippet vuln-line ruby_authn_ip_only
  BenchmarkResponse.ok('Access granted')
end
# vuln-code-snippet end ruby_authn_ip_only
