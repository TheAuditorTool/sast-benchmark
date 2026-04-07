require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_www_auth
def set_www_authenticate(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['WWW-Authenticate'] = "Bearer realm=\"#{req.param('realm')}\"" # vuln-code-snippet vuln-line ruby_headerinj_www_auth
  response
end
# vuln-code-snippet end ruby_headerinj_www_auth
