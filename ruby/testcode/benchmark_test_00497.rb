require_relative 'shared'
require 'net/http'
require 'uri'

def deliver_webhook(req)
  url = req.param('webhook_url')
  Net::HTTP.get(URI.parse(url))
  BenchmarkResponse.json({ ok: true })
end
