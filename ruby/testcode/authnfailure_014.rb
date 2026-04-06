require_relative 'shared'

# vuln-code-snippet start ruby_authn_apikey_header
def api_action_header(req)
  key = req.header('Authorization')
  return BenchmarkResponse.error('unauthorized', 401) unless key.start_with?('Bearer ') # vuln-code-snippet safe-line ruby_authn_apikey_header
  BenchmarkResponse.ok('action performed')
end
# vuln-code-snippet end ruby_authn_apikey_header
