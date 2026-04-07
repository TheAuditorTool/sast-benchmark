require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_refresh_header
def set_refresh_header(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Refresh'] = "0; url=#{req.param('url')}" # vuln-code-snippet vuln-line ruby_headerinj_refresh_header
  response
end
# vuln-code-snippet end ruby_headerinj_refresh_header
