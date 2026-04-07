require_relative 'shared'
require 'net/smtp'

# vuln-code-snippet start ruby_ssrf_smtp_host
def check_smtp(req)
  Net::SMTP.start(req.param('smtp_host'), 25) { |s| s.finish } # vuln-code-snippet vuln-line ruby_ssrf_smtp_host
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_smtp_host
