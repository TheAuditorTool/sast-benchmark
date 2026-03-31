require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_location
def redirect_to_url(req)
  input = req.param('url')
  BenchmarkResponse.new(302, '', { 'Location' => input })  # vuln-code-snippet vuln-line ruby_headerinj_location
end
# vuln-code-snippet end ruby_headerinj_location
