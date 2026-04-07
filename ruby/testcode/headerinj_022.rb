require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_set_cookie_raw
def set_raw_cookie(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Set-Cookie'] = "s=#{req.param('v')}; HttpOnly" # vuln-code-snippet vuln-line ruby_headerinj_set_cookie_raw
  response
end
# vuln-code-snippet end ruby_headerinj_set_cookie_raw
