require_relative 'shared'
require 'uri'

ALLOWED_HOSTS = %w[example.com www.example.com].freeze

# vuln-code-snippet start ruby_redirect_host_allowlist
def redirect_checked(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  return BenchmarkResponse.bad_request('invalid') unless parsed && ALLOWED_HOSTS.include?(parsed.host) # vuln-code-snippet safe-line ruby_redirect_host_allowlist
  BenchmarkResponse.redirect(url)
end
# vuln-code-snippet end ruby_redirect_host_allowlist
