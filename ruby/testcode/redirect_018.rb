require_relative 'shared'

# vuln-code-snippet start ruby_redirect_scheme_inject
def redirect_scheme_inject(req)
  url = "#{req.param('scheme')}://safe.example.com/home" # vuln-code-snippet vuln-line ruby_redirect_scheme_inject
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
# vuln-code-snippet end ruby_redirect_scheme_inject
