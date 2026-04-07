require 'net/http'
require_relative 'shared'

GH_TOKEN = "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

# vuln-code-snippet start ruby_hardcoded_github_pat
def list_repos_handler(req)
  uri = URI('https://api.github.com/user/repos')
  req_obj = Net::HTTP::Get.new(uri)
  req_obj['Authorization'] = "token #{GH_TOKEN}"  # vuln-code-snippet vuln-line ruby_hardcoded_github_pat
  response = Net::HTTP.start(uri.hostname, uri.port, use_ssl: true) { |http| http.request(req_obj) }
  BenchmarkResponse.json({ repos: JSON.parse(response.body) })
end
# vuln-code-snippet end ruby_hardcoded_github_pat
