require_relative 'shared'

def convert_document(req)
  filename = req.param('filename')
  result = system('convert_doc', filename, '--output', '/tmp/out.pdf')
  BenchmarkResponse.json({ success: result })
end
