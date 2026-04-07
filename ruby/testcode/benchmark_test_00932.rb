require_relative 'shared'

begin
  require 'erubi'
rescue LoadError
end

def handler(req)
  src = Erubi::Engine.new(req.param('tpl')).src
  result = eval(src)
  BenchmarkResponse.html(result.to_s)
end
