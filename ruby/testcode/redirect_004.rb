require_relative 'shared'

# vuln-code-snippet start ruby_redirect_allowlist
ALLOWED_HOSTS = %w[example.com www.example.com api.example.com].freeze

def allowlist_redirect(req)
  url = req.param('url')
  host = URI.parse(url).host rescue nil
  if ALLOWED_HOSTS.include?(host) # vuln-code-snippet safe-line ruby_redirect_allowlist
    BenchmarkResponse.redirect(url)
  else
    BenchmarkResponse.bad_request('host not allowed')
  end
end
# vuln-code-snippet end ruby_redirect_allowlist
