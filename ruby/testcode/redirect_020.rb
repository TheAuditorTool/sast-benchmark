require_relative 'shared'

# vuln-code-snippet start ruby_redirect_location_header_raw
def redirect_location_header_raw(req)
  response = BenchmarkResponse.ok('redirecting')
  response.headers['Location'] = req.param('next') # vuln-code-snippet vuln-line ruby_redirect_location_header_raw
  response
end
# vuln-code-snippet end ruby_redirect_location_header_raw
