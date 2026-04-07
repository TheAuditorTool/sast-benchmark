require_relative 'shared'

begin
  require 'kramdown'
rescue LoadError
end

def handler(req)
  output = Kramdown::Document.new(req.param('md')).to_html
  BenchmarkResponse.html(output)
end
