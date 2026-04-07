require_relative 'shared'
require 'net/http'

def fetch_with_timeouts(req)
  http = Net::HTTP.new('api.example.com', 443)
  http.open_timeout = 5
  http.read_timeout = 5
  http.max_retries = 0
  http.use_ssl = true
  http.get('/data')
  BenchmarkResponse.json({ ok: true })
end
