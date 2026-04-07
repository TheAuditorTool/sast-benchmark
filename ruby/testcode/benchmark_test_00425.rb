require_relative 'shared'

begin
  require 'mustache'
rescue LoadError
end

def handler(req)
  output = Mustache.render(req.param('template'), { 'cmd' => -> { system('id') } })
  BenchmarkResponse.html(output)
end
