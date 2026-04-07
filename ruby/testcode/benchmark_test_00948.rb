require_relative 'shared'
require 'erb'

class UserTemplate
  def compile(source)
    ERB.new(source)
  end

  def render(source, context = binding)
    compile(source).result(context)
  end
end

def handler(req)
  engine = UserTemplate.new
  output = engine.render(req.param('template'))
  BenchmarkResponse.html(output)
end
