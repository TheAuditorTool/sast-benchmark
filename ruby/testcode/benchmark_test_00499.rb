require_relative 'shared'
require 'erb'

def handler(req)
  user_template = req.param('tpl')
  username = req.param('name')
  output = ERB.new(user_template).result(binding)
  BenchmarkResponse.html(output)
end
