require_relative 'shared'

def handler(req)
  name = req.param('name')
  greeting = req.param('greeting', default: 'Hello')
  body = "<div class=\"greeting\">#{greeting}, #{name.html_safe}!</div>"
  BenchmarkResponse.html(body)
end
