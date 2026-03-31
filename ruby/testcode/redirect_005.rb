require_relative 'shared'

# vuln-code-snippet start ruby_redirect_header_location
def header_redirect(req)
  next_url = req.param('next')
  BenchmarkResponse.new(302, '', { 'Location' => next_url }) # vuln-code-snippet vuln-line ruby_redirect_header_location
end
# vuln-code-snippet end ruby_redirect_header_location
