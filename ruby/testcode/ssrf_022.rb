require_relative 'shared'
require 'net/ftp'

# vuln-code-snippet start ruby_ssrf_net_ftp
def fetch_ftp(req)
  Net::FTP.open(req.param('host')) { |ftp| ftp.list } # vuln-code-snippet vuln-line ruby_ssrf_net_ftp
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_net_ftp
