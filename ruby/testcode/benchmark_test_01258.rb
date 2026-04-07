require_relative 'shared'

begin
  require 'haml'
rescue LoadError
end

def handler(req)
  output = Haml::Engine.new(req.param('template')).render
  BenchmarkResponse.html(output)
end
