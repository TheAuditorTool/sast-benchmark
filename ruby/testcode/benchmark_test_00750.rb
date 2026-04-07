require_relative 'shared'
require 'net/http'
require 'uri'

BASE_API_URL = 'https://api.trusted.com/v1'.freeze

def fetch_api_path(req)
  path = File.basename(req.param('path'))
  Net::HTTP.get(URI.parse("#{BASE_API_URL}/#{path}"))
  BenchmarkResponse.json({ ok: true })
end
