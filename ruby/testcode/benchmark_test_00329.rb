require_relative 'shared'
require 'net/http'

def fetch_hardcoded_host(req)
  http = Net::HTTP.new('hardcoded.example.com', 443)
  http.use_ssl = true
  http.request(Net::HTTP::Get.new('/data'))
  BenchmarkResponse.json({ ok: true })
end
