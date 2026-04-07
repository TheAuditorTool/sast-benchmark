require_relative 'shared'

# vuln-code-snippet start ruby_authn_timing_attack
def authenticate_api_token(req)
  provided = req.header('X-Api-Token')
  stored = ENV.fetch('API_TOKEN', 'secret-token')
  return BenchmarkResponse.error('Unauthorized', 401) unless provided == stored # vuln-code-snippet vuln-line ruby_authn_timing_attack
  BenchmarkResponse.ok('authenticated')
end
# vuln-code-snippet end ruby_authn_timing_attack
