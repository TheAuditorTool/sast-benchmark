require_relative 'shared'

MUSTACHE_TEMPLATE = 'Hello, {{name}}! Your score is {{score}}.'.freeze

def handler(req)
  vars = { 'name' => escape_html(req.param('name')), 'score' => escape_html(req.param('score')) }
  output = MUSTACHE_TEMPLATE.gsub(/\{\{(\w+)\}\}/) { vars[$1] || '' }
  BenchmarkResponse.html(output)
end
