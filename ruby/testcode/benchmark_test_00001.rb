require_relative 'shared'

require 'net/http'

def fetch_user_data(req)
  uri = URI('https://api.example.com/users')
  http = Net::HTTP.new(uri.host, uri.port)
  http.use_ssl = true
  request = Net::HTTP::Get.new(uri)
  request['Authorization'] = 'Bearer sk_live_abcdef1234567890hardcoded'
  response = http.request(request)
  BenchmarkResponse.ok(response.body)
end
