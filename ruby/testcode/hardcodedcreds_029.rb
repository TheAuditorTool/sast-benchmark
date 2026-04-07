require 'net/http'
require_relative 'shared'

SF_CLIENT_SECRET = "xxxxxxxxxxxxxxxxxxxx"
SF_CLIENT_ID     = "3MVG9xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

# vuln-code-snippet start ruby_hardcoded_salesforce_secret
def salesforce_auth_handler(req)
  uri = URI('https://login.salesforce.com/services/oauth2/token')
  http_req = Net::HTTP::Post.new(uri)
  http_req.set_form_data(
    'grant_type'    => 'client_credentials',
    'client_id'     => SF_CLIENT_ID,
    'client_secret' => SF_CLIENT_SECRET  # vuln-code-snippet vuln-line ruby_hardcoded_salesforce_secret
  )
  response = Net::HTTP.start(uri.hostname, uri.port, use_ssl: true) { |http| http.request(http_req) }
  BenchmarkResponse.json({ token: JSON.parse(response.body)['access_token'] })
end
# vuln-code-snippet end ruby_hardcoded_salesforce_secret
