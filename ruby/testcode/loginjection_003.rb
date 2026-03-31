require_relative 'shared'

# vuln-code-snippet start ruby_loginj_multiline
def record_action(req)
  input = req.param('action')
  log_file = '/tmp/app.log'
  File.open(log_file, 'a') { |f| f.puts("ACTION: #{input}") }  # vuln-code-snippet vuln-line ruby_loginj_multiline
  BenchmarkResponse.ok('recorded')
end
# vuln-code-snippet end ruby_loginj_multiline
