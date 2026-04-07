require_relative 'shared'

begin
  require 'tilt'
rescue LoadError
end

def handler(req)
  tpl_str = req.param('template')
  output = Tilt['erb'].new { tpl_str }.render(Object.new)
  BenchmarkResponse.html(output)
end
