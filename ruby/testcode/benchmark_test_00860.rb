require_relative 'shared'
require 'erb'

def handler(req)
  output = ERB.new(req.param('template_str')).result(binding)
  BenchmarkResponse.html(output)
end
