require_relative 'shared'
require 'net/smtp'

def check_smtp(req)
  Net::SMTP.start(req.param('smtp_host'), 25) { |s| s.finish }
  BenchmarkResponse.json({ ok: true })
end
