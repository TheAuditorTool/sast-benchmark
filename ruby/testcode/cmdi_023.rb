require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_kernel_exec_str
def list_directory(req)
  dir = req.param('dir')
  Kernel.exec("ls -la #{dir}")  # vuln-code-snippet vuln-line ruby_cmdi_kernel_exec_str
  BenchmarkResponse.ok('done')
end
# vuln-code-snippet end ruby_cmdi_kernel_exec_str
