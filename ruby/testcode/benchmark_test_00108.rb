require_relative 'shared'

begin
  require 'redcarpet'
rescue LoadError
end

def handler(req)
  renderer = Redcarpet::Render::HTML.new
  output = Redcarpet::Markdown.new(renderer).render(req.param('content'))
  BenchmarkResponse.html(output)
end
