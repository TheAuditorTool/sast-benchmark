require_relative 'shared'
require 'net/http'
require 'uri'
require 'openssl'

WEBHOOK_SECRET = ENV.fetch('WEBHOOK_SECRET', 'changeme').freeze

# vuln-code-snippet start ruby_ssrf_webhook_signed_response
def deliver_signed_webhook(req)
  url = req.param('webhook_url')
  response_body = Net::HTTP.get(URI.parse(url))
  expected_sig = req.header('X-Signature')
  actual_sig = OpenSSL::HMAC.hexdigest('SHA256', WEBHOOK_SECRET, response_body)
  raise 'invalid signature' unless ActiveSupport::SecurityUtils.secure_compare(actual_sig, expected_sig) rescue nil # vuln-code-snippet safe-line ruby_ssrf_webhook_signed_response
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_webhook_signed_response
