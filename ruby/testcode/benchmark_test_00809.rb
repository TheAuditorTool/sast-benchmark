require_relative 'shared'

def handler(req)
  template_fragment = req.param('tpl')
  user_name = req.param('user')
  erb_src = "<div>Hello <%= #{template_fragment} %>, #{user_name}</div>"
  require 'erb'
  renderer = ERB.new(erb_src)
  output = renderer.result(binding)
  BenchmarkResponse.html(output)
end
