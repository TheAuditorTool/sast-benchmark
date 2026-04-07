require_relative 'shared'

# vuln-code-snippet start ruby_redirect_fragment_inject
def redirect_fragment_inject(req)
  url = "/page##{req.param('section')}" # vuln-code-snippet vuln-line ruby_redirect_fragment_inject
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
# vuln-code-snippet end ruby_redirect_fragment_inject
