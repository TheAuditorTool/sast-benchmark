require_relative 'shared'

def handler(req)
  username = req.param('username')
  message = req.param('message')
  safe_name = escape_html(username)
  safe_msg = escape_html(message)
  body = "<div><strong>#{safe_name}</strong>: #{safe_msg}</div>"
  BenchmarkResponse.html(body)
end
