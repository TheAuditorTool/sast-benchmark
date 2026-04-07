require_relative 'shared'

FILEMAP = { 1 => '/app/files/report.pdf', 2 => '/app/files/invoice.pdf' }.freeze

# vuln-code-snippet start ruby_pt_id_lookup
def read_by_id(req)
  path = FILEMAP.fetch(req.param('id').to_i)
  BenchmarkResponse.ok(File.read(path)) # vuln-code-snippet safe-line ruby_pt_id_lookup
end
# vuln-code-snippet end ruby_pt_id_lookup
