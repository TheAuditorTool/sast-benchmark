require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_set_cookie_name
def set_tracking_cookie(req)
  name = req.param('name')
  value = 'tracker_value_constant'
  response = BenchmarkResponse.ok('cookie set')
  response.headers['Set-Cookie'] = "#{name}=#{value}"  # vuln-code-snippet vuln-line ruby_headerinj_set_cookie_name
  response
end
# vuln-code-snippet end ruby_headerinj_set_cookie_name
