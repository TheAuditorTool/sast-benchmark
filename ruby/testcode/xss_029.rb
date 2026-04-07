require_relative 'shared'

# vuln-code-snippet start ruby_xss_srcdoc_inject
def render_preview_frame(req)
  html_content = req.param('html')
  iframe = content_tag(:iframe, '', srcdoc: html_content)  # vuln-code-snippet vuln-line ruby_xss_srcdoc_inject
  BenchmarkResponse.html(iframe)
end
# vuln-code-snippet end ruby_xss_srcdoc_inject
