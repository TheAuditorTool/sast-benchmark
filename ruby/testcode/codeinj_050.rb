require_relative 'shared'

begin
  require 'treetop'
rescue LoadError
end

# Stub Treetop grammar parse call.
# A real Treetop grammar compiles to a Ruby parser class that produces
# a syntax tree (SyntaxNode objects) — user input is never executed as Ruby code.
module TreetopStub
  class SyntaxNode
    attr_reader :text_value, :elements
    def initialize(text, elements = [])
      @text_value = text
      @elements = elements
    end
  end

  class ArithmeticParser
    def parse(expr)
      tokens = expr.scan(/\d+|[+\-*\/()]/)
      SyntaxNode.new(expr, tokens.map { |t| SyntaxNode.new(t) })
    end
  end
end

# vuln-code-snippet start ruby_codeinj_treetop_parse
def parse_expression(req)
  expr = req.param('expr')
  parser = TreetopStub::ArithmeticParser.new
  tree = parser.parse(expr) # vuln-code-snippet safe-line ruby_codeinj_treetop_parse
  BenchmarkResponse.json({ text: tree.text_value, node_count: tree.elements.size })
end
# vuln-code-snippet end ruby_codeinj_treetop_parse
