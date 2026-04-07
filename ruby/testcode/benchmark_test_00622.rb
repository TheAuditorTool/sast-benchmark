require_relative 'shared'

def log_file_write_crlf(req)
  File.open('/var/log/app.log', 'a') { |f| f.write("#{Time.now} #{req.param('action')}\n") }
  BenchmarkResponse.json({ ok: true })
end
