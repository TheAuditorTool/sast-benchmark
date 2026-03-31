require_relative 'shared'

# vuln-code-snippet start ruby_fi_kernel_load
def execute_script(req)
  path = req.param('path')
  Kernel.load(path) # vuln-code-snippet vuln-line ruby_fi_kernel_load
  BenchmarkResponse.ok("script executed")
end
# vuln-code-snippet end ruby_fi_kernel_load
