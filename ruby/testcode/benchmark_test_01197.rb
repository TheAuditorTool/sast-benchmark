require_relative 'shared'

begin
  require 'parser/current'
rescue LoadError
end

def parse_code(req)
  source = req.param('code')
  ast = Parser::CurrentRuby.parse(source)
  BenchmarkResponse.json({ ast: ast.to_s })
end
