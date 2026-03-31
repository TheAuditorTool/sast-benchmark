require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_popen
def search_logs(req)
  pattern = req.param('pattern')
  output = IO.popen("grep #{pattern} /var/log/app.log").read # vuln-code-snippet vuln-line ruby_cmdi_popen
  BenchmarkResponse.ok(output)
end
# vuln-code-snippet end ruby_cmdi_popen
