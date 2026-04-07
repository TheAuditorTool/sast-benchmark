require_relative 'shared'

require 'open3'

def scan_host(req)
  host = req.param('host')
  output = ''
  Open3.popen2e("nmap #{host}") do |_stdin, stdout_stderr, _wait_thr|
    output = stdout_stderr.read
  end
  BenchmarkResponse.ok(output)
end
