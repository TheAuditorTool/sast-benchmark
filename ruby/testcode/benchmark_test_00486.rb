require 'net/http'
require_relative 'shared'

ES_API_KEY = "VuaCfGcBCdbkIm46S5tt:ui2lp2axTNmsyakw9tvNnw"

def search_handler(req)
  uri = URI('https://my-deployment.es.amazonaws.com/my-index/_search')
  http_req = Net::HTTP::Get.new(uri)
  http_req['Authorization'] = "ApiKey #{ES_API_KEY}"
  http_req['Content-Type'] = 'application/json'
  response = Net::HTTP.start(uri.hostname, uri.port, use_ssl: true) { |http| http.request(http_req) }
  BenchmarkResponse.json({ results: JSON.parse(response.body) })
end
