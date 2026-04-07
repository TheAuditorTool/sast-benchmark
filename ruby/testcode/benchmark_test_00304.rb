require_relative 'shared'

def render_sanitized_html(req)
  html_input = req.param('html')
  clean = Loofah.fragment(html_input).scrub!(:strip).to_s
  BenchmarkResponse.html(clean)
end
