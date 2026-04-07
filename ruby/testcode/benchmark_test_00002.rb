require 'net/http'
require_relative 'shared'

HEROKU_API_KEY = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"

def list_heroku_apps_handler(req)
  uri = URI('https://api.heroku.com/apps')
  http_req = Net::HTTP::Get.new(uri)
  http_req['Authorization'] = "Bearer #{HEROKU_API_KEY}"
  http_req['Accept'] = 'application/vnd.heroku+json; version=3'
  response = Net::HTTP.start(uri.hostname, uri.port, use_ssl: true) { |http| http.request(http_req) }
  BenchmarkResponse.json({ apps: JSON.parse(response.body) })
end
