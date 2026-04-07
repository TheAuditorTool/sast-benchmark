require_relative 'shared'
require 'erb'

def handler(req)
  output = ERB.new(req.param('body')).result(binding)
  BenchmarkResponse.html(output)
end
