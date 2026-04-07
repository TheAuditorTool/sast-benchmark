require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_cors_origin
def set_cors_origin(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Access-Control-Allow-Origin'] = req.param('origin') # vuln-code-snippet vuln-line ruby_headerinj_cors_origin
  response
end
# vuln-code-snippet end ruby_headerinj_cors_origin
