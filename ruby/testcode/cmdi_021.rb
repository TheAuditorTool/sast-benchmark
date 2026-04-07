require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_io_popen_pipe
def run_pipe_command(req)
  cmd = req.param('cmd')
  output = IO.popen("| #{cmd}") { |io| io.read }  # vuln-code-snippet vuln-line ruby_cmdi_io_popen_pipe
  BenchmarkResponse.ok(output)
end
# vuln-code-snippet end ruby_cmdi_io_popen_pipe
