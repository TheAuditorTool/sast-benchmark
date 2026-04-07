require_relative 'shared'

begin
  require 'haml'
rescue LoadError
end

def handler(req)
  output = Haml::Engine.new(File.read('templates/email.haml')).render(Object.new, name: req.param('name'))
  BenchmarkResponse.html(output)
end
