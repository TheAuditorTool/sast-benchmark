require_relative 'shared'

def chmod_file(req)
  File.chmod(0644, req.param('path'))
  BenchmarkResponse.json({ ok: true })
end
