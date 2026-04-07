require_relative 'shared'

begin
  require 'slim'
rescue LoadError
end

def handler(req)
  tpl_str = req.param('tpl')
  output = Slim::Template.new { tpl_str }.render
  BenchmarkResponse.html(output)
end
