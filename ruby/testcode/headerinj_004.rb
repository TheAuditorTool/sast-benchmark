require_relative 'shared'

ALLOWED_THEMES = %w[light dark high-contrast].freeze

# vuln-code-snippet start ruby_headerinj_allowlist_header
def set_theme_header(req)
  requested = req.param('theme')
  theme = ALLOWED_THEMES.include?(requested) ? requested : 'light'
  headers = { 'X-Theme' => theme }  # vuln-code-snippet safe-line ruby_headerinj_allowlist_header
  BenchmarkResponse.new(200, 'ok', headers)
end
# vuln-code-snippet end ruby_headerinj_allowlist_header
