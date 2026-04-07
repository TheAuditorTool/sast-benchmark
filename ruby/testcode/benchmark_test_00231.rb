require_relative 'shared'

ALLOWED_XSS_LINK_HOSTS = %w[app.example.com cdn.example.com].freeze

def render_validated_link(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  safe_url = (parsed && parsed.scheme == 'https' && ALLOWED_XSS_LINK_HOSTS.include?(parsed.host)) ? url : '#'
  html = link_to('Visit', safe_url)
  BenchmarkResponse.html(html)
end
