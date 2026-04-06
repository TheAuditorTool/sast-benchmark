require_relative 'shared'

# vuln-code-snippet start ruby_authn_timing_compare
def verify_api_key(req)
  provided = req.header('X-API-Key')
  expected = ENV.fetch('API_KEY', '')
  if provided == expected # vuln-code-snippet vuln-line ruby_authn_timing_compare
    BenchmarkResponse.ok('authenticated')
  else
    BenchmarkResponse.error('unauthorized', 401)
  end
end
# vuln-code-snippet end ruby_authn_timing_compare
