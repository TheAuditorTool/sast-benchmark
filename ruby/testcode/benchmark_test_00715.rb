require_relative 'shared'

begin
  require 'mustache'
rescue LoadError
end

def handler(req)
  output = Mustache.render('Hello {{name}}', { name: req.param('name') })
  BenchmarkResponse.html(output)
end
