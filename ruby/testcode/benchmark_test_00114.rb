require_relative 'shared'

SAFE_DIR   = File.expand_path('plugins').freeze
ALLOWLIST  = %w[pdf csv xml].freeze

def handler(req)
  f = req.param('f')
  raise 'invalid' unless File.extname(f) == '.rb' && ALLOWLIST.include?(File.basename(f, '.rb'))
  load(File.join(SAFE_DIR, f))
  BenchmarkResponse.json({ ok: true })
end
