require_relative 'shared'

def render_back_link(req)
  url = req.param('url')
  html = link_to('Click here', url)
  BenchmarkResponse.html(html)
end
