require_relative 'shared'
require 'erb'

def handler(req)
  expr = req.param('expr')
  erb_str = "<%= #{expr} %>"
  output = ERB.new(erb_str).result(binding)
  BenchmarkResponse.html(output)
end
