require_relative 'shared'

FILEMAP = { 1 => '/app/files/report.pdf', 2 => '/app/files/invoice.pdf' }.freeze

def read_by_id(req)
  path = FILEMAP.fetch(req.param('id').to_i)
  BenchmarkResponse.ok(File.read(path))
end
