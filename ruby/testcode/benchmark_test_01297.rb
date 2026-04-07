require_relative 'shared'

def render_greeting(req)
  name = req.param('name')
  BenchmarkResponse.html("<html><body><h1>Hello, #{name}!</h1></body></html>")
end
