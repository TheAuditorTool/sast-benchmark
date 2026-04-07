require_relative 'shared'

def render_notification(req)
  username = req.param('username')
  count = req.param('count')
  template_str = 'Hello <%= data[:username] %>, you have <%= data[:count] %> messages.'
  output = ERB.new(template_str).result_with_hash(data: { username: username, count: count })
  BenchmarkResponse.html(output)
end
