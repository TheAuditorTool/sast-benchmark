require_relative 'shared'
require 'net/http'
require 'uri'

ALLOWED_URLS = {
  1 => 'https://api.example.com',
  2 => 'https://cdn.example.com'
}.freeze

def fetch_by_id(req)
  url = ALLOWED_URLS.fetch(req.param('id').to_i)
  Net::HTTP.get(URI.parse(url))
  BenchmarkResponse.json({ ok: true })
end
