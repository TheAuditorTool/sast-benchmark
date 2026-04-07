require_relative 'shared'

SAFE_DIR = File.expand_path('plugins').freeze

# vuln-code-snippet start ruby_fi_basename_only_load
def handler(req)
  f = File.join(SAFE_DIR, File.basename(req.param('f').gsub('..', '')) + '.rb')
  load(f) # vuln-code-snippet safe-line ruby_fi_basename_only_load
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_basename_only_load
