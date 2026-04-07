require_relative 'shared'

# vuln-code-snippet start ruby_loginj_file_write_crlf
def log_file_write_crlf(req)
  File.open('/var/log/app.log', 'a') { |f| f.write("#{Time.now} #{req.param('action')}\n") } # vuln-code-snippet vuln-line ruby_loginj_file_write_crlf
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_file_write_crlf
