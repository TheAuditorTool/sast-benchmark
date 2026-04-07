require_relative 'shared'

# vuln-code-snippet start ruby_xss_json_safe_embed
def render_page_with_data(req)
  val = req.param('val')
  # to_json escapes special chars; html_escape ensures </script> injection blocked
  safe_json = val.to_json.gsub('<', '\u003c').gsub('>', '\u003e').gsub('&', '\u0026')
  html = "<script>var data = #{safe_json};</script>"  # vuln-code-snippet safe-line ruby_xss_json_safe_embed
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_xss_json_safe_embed
