require_relative 'shared'

SAFE_DIR = File.expand_path('plugins').freeze

def handler(req)
  f = File.join(SAFE_DIR, File.basename(req.param('f').gsub('..', '')) + '.rb')
  load(f)
  BenchmarkResponse.json({ ok: true })
end
