require_relative 'shared'
require 'erb'

def handler(req)
  inline_tpl = req.param('template')
  output = ERB.new(inline_tpl).result(binding)
  BenchmarkResponse.html(output)
end
