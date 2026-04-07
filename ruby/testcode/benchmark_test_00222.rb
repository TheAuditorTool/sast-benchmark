require_relative 'shared'

ALLOWED_HOSTS = %w[example.com www.example.com api.example.com].freeze

def allowlist_redirect(req)
  url = req.param('url')
  host = URI.parse(url).host rescue nil
  if ALLOWED_HOSTS.include?(host)
    BenchmarkResponse.redirect(url)
  else
    BenchmarkResponse.bad_request('host not allowed')
  end
end
