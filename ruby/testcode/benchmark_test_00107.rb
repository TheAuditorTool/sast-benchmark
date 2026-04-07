require_relative 'shared'
require 'erb'

def handler(req)
  name  = escape_html(req.param('name'))
  score = escape_html(req.param('score'))
  template = ERB.new(File.read('templates/report.html.erb'))
  output = template.result(binding)
  BenchmarkResponse.html(output)
end
