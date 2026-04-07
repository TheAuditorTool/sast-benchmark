require_relative 'shared'

# vuln-code-snippet start ruby_xss_loofah_strict
def render_sanitized_html(req)
  html_input = req.param('html')
  clean = Loofah.fragment(html_input).scrub!(:strip).to_s  # vuln-code-snippet safe-line ruby_xss_loofah_strict
  BenchmarkResponse.html(clean)
end
# vuln-code-snippet end ruby_xss_loofah_strict
