require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_exec_interp
def convert_image(req)
  filename = req.post('filename')
  exec("convert #{filename} output.png") # vuln-code-snippet vuln-line ruby_cmdi_exec_interp
  BenchmarkResponse.ok("conversion started")
end
# vuln-code-snippet end ruby_cmdi_exec_interp
