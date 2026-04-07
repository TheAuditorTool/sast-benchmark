require_relative 'shared'
require 'uri'

ALLOWED = %w[app.example.com api.example.com].freeze

# vuln-code-snippet start ruby_redirect_host_allowlist2
def redirect_host_allowlist(req)
  url = req.param('url')
  host = URI.parse(url).host
  dest = ALLOWED.include?(host) ? url : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '') # vuln-code-snippet safe-line ruby_redirect_host_allowlist2
end
# vuln-code-snippet end ruby_redirect_host_allowlist2
