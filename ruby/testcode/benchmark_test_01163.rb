require_relative 'shared'

GREETING_TEMPLATE = 'Welcome, {{name}}!'.freeze

def handler(req)
  name = escape_html(req.param('name'))
  output = GREETING_TEMPLATE.gsub('{{name}}', name)
  BenchmarkResponse.html(output)
end
