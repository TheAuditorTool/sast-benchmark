require_relative 'shared'

ALLOWED_XSS_LINK_HOSTS = %w[app.example.com cdn.example.com].freeze

# vuln-code-snippet start ruby_xss_link_validated
def render_validated_link(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  safe_url = (parsed && parsed.scheme == 'https' && ALLOWED_XSS_LINK_HOSTS.include?(parsed.host)) ? url : '#'
  html = link_to('Visit', safe_url)  # vuln-code-snippet safe-line ruby_xss_link_validated
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_link_validated
