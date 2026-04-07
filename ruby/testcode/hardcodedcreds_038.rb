require 'net/http'
require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_figaro
def api_request_handler(req)
  api_key = Figaro.env.api_key!  # vuln-code-snippet safe-line ruby_hardcoded_figaro
  uri = URI('https://api.example.com/data')
  http_req = Net::HTTP::Get.new(uri)
  http_req['X-API-Key'] = api_key
  response = Net::HTTP.start(uri.hostname, uri.port, use_ssl: true) { |http| http.request(http_req) }
  BenchmarkResponse.json({ data: JSON.parse(response.body) })
end
# vuln-code-snippet end ruby_hardcoded_figaro
