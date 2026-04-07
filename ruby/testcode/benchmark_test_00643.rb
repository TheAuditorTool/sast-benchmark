require_relative 'shared'

def handler(req)
  base = File.expand_path('modules')
  rel  = req.param('mod')
  p    = Pathname.new(base).join(rel + '.rb').cleanpath
  raise 'path traversal detected' unless p.to_s.start_with?(base)
  load(p.to_s)
  BenchmarkResponse.json({ ok: true })
end
