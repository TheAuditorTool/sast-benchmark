require_relative 'shared'

require 'open3'

ALLOWED_SCAN_HOSTS = %w[192.168.1.1 192.168.1.2 10.0.0.1].freeze

# vuln-code-snippet start ruby_cmdi_open3_popen2e_array
def scan_host_safe(req)
  host = req.param('host')
  allowlisted_host = ALLOWED_SCAN_HOSTS.include?(host) ? host : '127.0.0.1'
  output = ''
  Open3.popen2e('nmap', '-sn', allowlisted_host) do |_stdin, stdout_stderr, _wait_thr|  # vuln-code-snippet safe-line ruby_cmdi_open3_popen2e_array
    output = stdout_stderr.read
  end
  BenchmarkResponse.ok(output)
end
# vuln-code-snippet end ruby_cmdi_open3_popen2e_array
