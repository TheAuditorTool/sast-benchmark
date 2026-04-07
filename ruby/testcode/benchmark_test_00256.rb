require_relative 'shared'
require 'erb'

def handler(req)
  template = req.param('template')
  output = ERB.new(template).result
  BenchmarkResponse.html(output)
end
