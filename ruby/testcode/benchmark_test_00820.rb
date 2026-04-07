require_relative 'shared'
require 'net/http'
require 'uri'

def fetch_https_only(req)
  uri = URI.parse(req.param('url'))
  raise 'only https allowed' unless uri.scheme == 'https'
  Net::HTTP.get(uri)
  BenchmarkResponse.json({ ok: true })
end
