require_relative 'shared'

# vuln-code-snippet start ruby_pt_file_chmod
def chmod_file(req)
  File.chmod(0644, req.param('path')) # vuln-code-snippet vuln-line ruby_pt_file_chmod
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_pt_file_chmod
