require_relative 'shared'

# vuln-code-snippet start ruby_redirect_allow_other_host
def redirect_allow_other_host(req)
  # simulate allow_other_host: true
  url = req.param('url')
  BenchmarkResponse.new(302, { 'Location' => url }, '') # vuln-code-snippet vuln-line ruby_redirect_allow_other_host
end
# vuln-code-snippet end ruby_redirect_allow_other_host
