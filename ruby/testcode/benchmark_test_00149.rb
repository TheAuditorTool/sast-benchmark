require_relative 'shared'
require 'uri'

ALLOWED_HOSTS = %w[example.com www.example.com].freeze

def redirect_checked(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  return BenchmarkResponse.bad_request('invalid') unless parsed && ALLOWED_HOSTS.include?(parsed.host)
  BenchmarkResponse.redirect(url)
end
