require_relative 'shared'

# vuln-code-snippet start ruby_loginj_stdout_puts
def log_stdout_puts(req)
  $stdout.puts("[AUDIT] user=#{req.param('id')} action=login") # vuln-code-snippet vuln-line ruby_loginj_stdout_puts
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_stdout_puts
