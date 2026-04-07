require_relative 'shared'
require 'uri'

ALLOWED = %w[app.example.com api.example.com].freeze

def redirect_host_allowlist(req)
  url = req.param('url')
  host = URI.parse(url).host
  dest = ALLOWED.include?(host) ? url : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
