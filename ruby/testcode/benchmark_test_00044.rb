require 'net/http'
require_relative 'shared'

def api_request_handler(req)
  api_key = Figaro.env.api_key!
  uri = URI('https://api.example.com/data')
  http_req = Net::HTTP::Get.new(uri)
  http_req['X-API-Key'] = api_key
  response = Net::HTTP.start(uri.hostname, uri.port, use_ssl: true) { |http| http.request(http_req) }
  BenchmarkResponse.json({ data: JSON.parse(response.body) })
end
