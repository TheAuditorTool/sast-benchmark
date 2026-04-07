require_relative 'shared'
require 'net/http'
require 'uri'

def check_service_health(req)
  response = Net::HTTP.get(URI("https://api.internal.com/status"))
  BenchmarkResponse.ok(response)
end
