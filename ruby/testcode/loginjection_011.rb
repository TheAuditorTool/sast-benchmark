require_relative 'shared'

# vuln-code-snippet start ruby_loginj_file_append
def log_to_file(req)
  message = req.param('message')
  File.open('application.log', 'a') { |f| f.puts(message) } # vuln-code-snippet vuln-line ruby_loginj_file_append
  BenchmarkResponse.ok('logged')
end
# vuln-code-snippet end ruby_loginj_file_append
