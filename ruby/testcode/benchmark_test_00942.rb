require_relative 'shared'

def handler(req)
  name = req.param('name')
  role = req.cookie('role')
  html = "<p>Welcome, " + name + "!</p>"
  html += "<span class=\"role\">" + role + "</span>"
  BenchmarkResponse.html(html)
end
