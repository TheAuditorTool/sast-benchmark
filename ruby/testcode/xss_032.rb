require_relative 'shared'

def normalize_input(val)
  val.to_s.strip  # to_s does NOT escape HTML — taint survives
end

# vuln-code-snippet start ruby_xss_multihop_html
def render_notification(req)
  raw_msg = req.param('message')
  normalized = normalize_input(raw_msg)   # taint survives to_s + strip
  html = raw(normalized)  # vuln-code-snippet vuln-line ruby_xss_multihop_html
  BenchmarkResponse.html("<div class='notification'>#{html}</div>")
end
# vuln-code-snippet end ruby_xss_multihop_html
