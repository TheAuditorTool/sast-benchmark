require 'net/http'
require 'json'
require_relative 'shared'

WEBHOOK_URL = "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX"

# vuln-code-snippet start ruby_hardcoded_slack_webhook
def notify_slack_handler(req)
  uri = URI(WEBHOOK_URL)  # vuln-code-snippet vuln-line ruby_hardcoded_slack_webhook
  Net::HTTP.post(uri, { text: 'Deploy complete' }.to_json, 'Content-Type' => 'application/json')
  BenchmarkResponse.ok('notification sent')
end
# vuln-code-snippet end ruby_hardcoded_slack_webhook
