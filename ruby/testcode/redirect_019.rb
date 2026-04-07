require_relative 'shared'

# vuln-code-snippet start ruby_redirect_host_concat2
def redirect_host_concat2(req)
  url = "https://#{req.param('host')}/dashboard" # vuln-code-snippet vuln-line ruby_redirect_host_concat2
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
# vuln-code-snippet end ruby_redirect_host_concat2
