require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_process_spawn_str
def ping_host(req)
  host = req.param('host')
  pid = Process.spawn("ping -c 4 #{host}")  # vuln-code-snippet vuln-line ruby_cmdi_process_spawn_str
  Process.wait(pid)
  BenchmarkResponse.json({ pid: pid })
end
# vuln-code-snippet end ruby_cmdi_process_spawn_str
