require 'net/http'
require_relative 'shared'

GH_TOKEN = "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

def list_repos_handler(req)
  uri = URI('https://api.github.com/user/repos')
  req_obj = Net::HTTP::Get.new(uri)
  req_obj['Authorization'] = "token #{GH_TOKEN}"
  response = Net::HTTP.start(uri.hostname, uri.port, use_ssl: true) { |http| http.request(req_obj) }
  BenchmarkResponse.json({ repos: JSON.parse(response.body) })
end
