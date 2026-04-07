require_relative 'shared'

require 'open3'

# vuln-code-snippet start ruby_cmdi_open3_popen2e_str
def scan_host(req)
  host = req.param('host')
  output = ''
  Open3.popen2e("nmap #{host}") do |_stdin, stdout_stderr, _wait_thr|  # vuln-code-snippet vuln-line ruby_cmdi_open3_popen2e_str
    output = stdout_stderr.read
  end
  BenchmarkResponse.ok(output)
end
# vuln-code-snippet end ruby_cmdi_open3_popen2e_str
