require_relative 'shared'

begin
  require 'erb'
rescue LoadError
end

def render_template(req)
  tpl = req.param('tpl')
  renderer = ERB.new(tpl)
  output = renderer.result(binding)
  BenchmarkResponse.html(output)
end
