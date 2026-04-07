require_relative 'shared'
require 'net/http'
require 'uri'

SAFE_API_BASE = 'https://api.trusted.com/v1'.freeze

def fetch_path_only(req)
  path = File.basename(req.param('path').gsub('..', ''))
  Net::HTTP.get(URI.parse("#{SAFE_API_BASE}/#{path}"))
  BenchmarkResponse.json({ ok: true })
end
