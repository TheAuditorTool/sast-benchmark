require_relative 'shared'

# vuln-code-snippet start ruby_authn_no_confirm
def login_no_confirm(req)
  username = req.param('username')
  # sign_in(user) without checking email confirmation
  BenchmarkResponse.ok("signed in: #{username}") # vuln-code-snippet vuln-line ruby_authn_no_confirm
end
# vuln-code-snippet end ruby_authn_no_confirm
