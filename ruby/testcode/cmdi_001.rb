require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_backtick
def list_directory(req)
  path = req.param('path')
  result = `ls #{path}` # vuln-code-snippet vuln-line ruby_cmdi_backtick
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_backtick
