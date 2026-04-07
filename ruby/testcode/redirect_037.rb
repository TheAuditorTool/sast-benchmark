require_relative 'shared'

# vuln-code-snippet start ruby_redirect_login_fixed
def redirect_login_fixed(req)
  BenchmarkResponse.new(302, { 'Location' => '/dashboard' }, '') # vuln-code-snippet safe-line ruby_redirect_login_fixed
end
# vuln-code-snippet end ruby_redirect_login_fixed
