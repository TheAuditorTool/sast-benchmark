require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_shell_exec_concat
def archive_report(req)
  report_name = req.param('report')
  Kernel.system("tar -czf /tmp/archive.tar.gz " + report_name) # vuln-code-snippet vuln-line ruby_cmdi_shell_exec_concat
  BenchmarkResponse.ok("archive created")
end
# vuln-code-snippet end ruby_cmdi_shell_exec_concat
