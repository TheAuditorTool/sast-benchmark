require_relative 'shared'

def render_user_greeting(req)
  name = req.param('name')
  html = javascript_tag("var greeting = 'Hello, #{name}!';")
  BenchmarkResponse.html(html)
end
