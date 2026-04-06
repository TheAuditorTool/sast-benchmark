require_relative 'shared'

# vuln-code-snippet start ruby_authn_apikey_query
def api_action_query(req)
  key = req.param('api_key')
  return BenchmarkResponse.error('unauthorized', 401) if key.empty?
  BenchmarkResponse.ok('action performed') # vuln-code-snippet vuln-line ruby_authn_apikey_query
end
# vuln-code-snippet end ruby_authn_apikey_query
