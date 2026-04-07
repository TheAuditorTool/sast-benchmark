require_relative 'shared'

begin
  require 'parser/current'
rescue LoadError
end

# vuln-code-snippet start ruby_codeinj_ast_parse_only
def parse_code(req)
  source = req.param('code')
  ast = Parser::CurrentRuby.parse(source) # vuln-code-snippet safe-line ruby_codeinj_ast_parse_only
  BenchmarkResponse.json({ ast: ast.to_s })
end
# vuln-code-snippet end ruby_codeinj_ast_parse_only
