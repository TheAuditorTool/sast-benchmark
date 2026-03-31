require_relative 'shared'

# vuln-code-snippet start ruby_pt_file_write
def append_log_entry(req)
  logfile = req.param('logfile')
  content = req.post('content')
  File.write("/logs/#{logfile}", content) # vuln-code-snippet vuln-line ruby_pt_file_write
  BenchmarkResponse.ok("Log entry written")
end
# vuln-code-snippet end ruby_pt_file_write
