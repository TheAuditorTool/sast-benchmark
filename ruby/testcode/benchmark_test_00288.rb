require_relative 'shared'

begin
  require 'slim'
rescue LoadError
end

def handler(req)
  output = Slim::Template.new('views/page.slim').render(Object.new)
  BenchmarkResponse.html(output)
end
