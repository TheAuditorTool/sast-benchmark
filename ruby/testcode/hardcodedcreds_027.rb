require 'net/http'
require_relative 'shared'

PAYPAL_SECRET = "EBWKjlELKMYqRNQ8sYvFVVL9g4IcPQF"
PAYPAL_CLIENT_ID = "AeB0EKxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

# vuln-code-snippet start ruby_hardcoded_paypal_secret
def paypal_token_handler(req)
  uri = URI('https://api.sandbox.paypal.com/v1/oauth2/token')
  http_req = Net::HTTP::Post.new(uri)
  http_req.basic_auth(PAYPAL_CLIENT_ID, PAYPAL_SECRET)  # vuln-code-snippet vuln-line ruby_hardcoded_paypal_secret
  http_req.set_form_data('grant_type' => 'client_credentials')
  response = Net::HTTP.start(uri.hostname, uri.port, use_ssl: true) { |http| http.request(http_req) }
  BenchmarkResponse.json({ token: JSON.parse(response.body)['access_token'] })
end
# vuln-code-snippet end ruby_hardcoded_paypal_secret
