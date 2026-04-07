require_relative 'shared'

begin
  require 'tilt'
rescue LoadError
end

def handler(req)
  tpl_str = req.param('haml_tpl')
  output = Tilt['haml'].new { tpl_str }.render
  BenchmarkResponse.html(output)
end
