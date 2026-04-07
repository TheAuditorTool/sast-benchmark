require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_process_spawn_array
def ping_host_safe(req)
  host = req.param('host').gsub(/[^a-z0-9.\-]/i, '')
  pid = Process.spawn('ping', '-c', '1', host)  # vuln-code-snippet safe-line ruby_cmdi_process_spawn_array
  Process.wait(pid)
  BenchmarkResponse.json({ pid: pid })
end
# vuln-code-snippet end ruby_cmdi_process_spawn_array
