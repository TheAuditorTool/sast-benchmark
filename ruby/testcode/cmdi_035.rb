require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_kernel_exec_array
def list_directory_safe(req)
  dir = req.param('dir')
  # Array form bypasses shell — no shell interpolation
  Kernel.exec('ls', '-la', '--', dir)  # vuln-code-snippet safe-line ruby_cmdi_kernel_exec_array
  BenchmarkResponse.ok('done')
end
# vuln-code-snippet end ruby_cmdi_kernel_exec_array
