require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_exec_array
def convert_image_safe(req)
  filename = req.post('filename')
  exec("convert", filename, "output.png") # vuln-code-snippet safe-line ruby_cmdi_exec_array
  BenchmarkResponse.ok("conversion started")
end
# vuln-code-snippet end ruby_cmdi_exec_array
