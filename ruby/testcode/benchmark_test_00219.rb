require_relative 'shared'

def create_symlink(req)
  File.symlink(req.param('target'), '/app/links/mylink')
  BenchmarkResponse.json({ ok: true })
end
