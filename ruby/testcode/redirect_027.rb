require_relative 'shared'

# vuln-code-snippet start ruby_redirect_after_login_open
def redirect_after_login_open(req)
  stored_url = req.param('next')
  BenchmarkResponse.new(302, { 'Location' => stored_url }, '') # vuln-code-snippet vuln-line ruby_redirect_after_login_open
end
# vuln-code-snippet end ruby_redirect_after_login_open
