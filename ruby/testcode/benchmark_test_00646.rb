require_relative 'shared'

begin
  require 'tilt'
rescue LoadError
end

def handler(req)
  output = Tilt.new('views/fixed.html.erb').render(Object.new, { name: req.param('name') })
  BenchmarkResponse.html(output)
end
