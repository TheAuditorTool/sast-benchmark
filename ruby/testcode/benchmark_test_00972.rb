require_relative 'shared'
require 'net/ftp'

def fetch_ftp(req)
  Net::FTP.open(req.param('host')) { |ftp| ftp.list }
  BenchmarkResponse.json({ ok: true })
end
