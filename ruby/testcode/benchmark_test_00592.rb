require 'net/http'
require_relative 'shared'

def notify_slack_safe_handler(req)
  webhook_url = ENV.fetch('SLACK_WEBHOOK') { raise 'SLACK_WEBHOOK not set' }
  uri = URI(webhook_url)
  Net::HTTP.post(uri, { text: req[:message] }.to_json, 'Content-Type' => 'application/json')
  BenchmarkResponse.ok('notification sent')
end
