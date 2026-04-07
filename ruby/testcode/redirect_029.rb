require_relative 'shared'

# vuln-code-snippet start ruby_redirect_subdomain_bypass
def redirect_subdomain_bypass(req)
  url = "https://#{req.param('sub')}.example.com/home" # vuln-code-snippet vuln-line ruby_redirect_subdomain_bypass
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
# vuln-code-snippet end ruby_redirect_subdomain_bypass
