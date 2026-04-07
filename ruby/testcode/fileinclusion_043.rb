require_relative 'shared'

SAFE_DIR   = File.expand_path('plugins').freeze
ALLOWLIST  = %w[pdf csv xml].freeze

# vuln-code-snippet start ruby_fi_ext_check_rb_only
def handler(req)
  f = req.param('f')
  raise 'invalid' unless File.extname(f) == '.rb' && ALLOWLIST.include?(File.basename(f, '.rb')) # vuln-code-snippet safe-line ruby_fi_ext_check_rb_only
  load(File.join(SAFE_DIR, f))
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_ext_check_rb_only
