require_relative 'shared'

def render_user_init_script(req)
  name = req.param('name')
  html = javascript_tag("var username = '#{j(name)}';")
  BenchmarkResponse.html(html)
end
