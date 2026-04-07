require 'net/http'
require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_env_fetch_desc
def notify_slack_safe_handler(req)
  webhook_url = ENV.fetch('SLACK_WEBHOOK') { raise 'SLACK_WEBHOOK not set' }  # vuln-code-snippet safe-line ruby_hardcoded_env_fetch_desc
  uri = URI(webhook_url)
  Net::HTTP.post(uri, { text: req[:message] }.to_json, 'Content-Type' => 'application/json')
  BenchmarkResponse.ok('notification sent')
end
# vuln-code-snippet end ruby_hardcoded_env_fetch_desc
