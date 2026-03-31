require_relative 'shared'

require 'net/http'

# vuln-code-snippet start ruby_hardcoded_env_api
def fetch_user_data(req)
  uri = URI('https://api.example.com/users')
  http = Net::HTTP.new(uri.host, uri.port)
  http.use_ssl = true
  request = Net::HTTP::Get.new(uri)
  request['Authorization'] = "Bearer #{ENV['API_KEY']}"  # vuln-code-snippet safe-line ruby_hardcoded_env_api
  response = http.request(request)
  BenchmarkResponse.ok(response.body)
end
# vuln-code-snippet end ruby_hardcoded_env_api
