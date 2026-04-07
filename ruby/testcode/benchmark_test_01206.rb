require_relative 'shared'

def render_preview_frame(req)
  html_content = req.param('html')
  iframe = content_tag(:iframe, '', srcdoc: html_content)
  BenchmarkResponse.html(iframe)
end
