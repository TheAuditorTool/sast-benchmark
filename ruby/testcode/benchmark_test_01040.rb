require_relative 'shared'

def render_nav_link(req)
  path = req.param('path')
  safe_url = url_for(only_path: true, path: path)
  html = link_to('Navigate', safe_url)
  BenchmarkResponse.html(html)
end
