require_relative 'shared'

# vuln-code-snippet start ruby_loginj_stderr_write
def log_stderr(req)
  user_input = req.param('message')
  $stderr.write("Error: #{user_input}\n") # vuln-code-snippet vuln-line ruby_loginj_stderr_write
  BenchmarkResponse.ok('error logged')
end
# vuln-code-snippet end ruby_loginj_stderr_write
