require_relative 'shared'

# vuln-code-snippet start ruby_pt_file_symlink
def create_symlink(req)
  File.symlink(req.param('target'), '/app/links/mylink') # vuln-code-snippet vuln-line ruby_pt_file_symlink
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_pt_file_symlink
