require_relative 'shared'

# vuln-code-snippet start ruby_fi_kernel_load_abs
def handler(req)
  Kernel.load("/app/extensions/#{req.param('ext')}.rb") # vuln-code-snippet vuln-line ruby_fi_kernel_load_abs
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_kernel_load_abs
