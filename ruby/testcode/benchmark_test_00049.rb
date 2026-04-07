require_relative 'shared'

def log_stdout_puts(req)
  $stdout.puts("[AUDIT] user=#{req.param('id')} action=login")
  BenchmarkResponse.json({ ok: true })
end
