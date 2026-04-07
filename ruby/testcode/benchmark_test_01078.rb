require_relative 'shared'

begin
  require 'treetop'
rescue LoadError
end

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

def parse_expression(req)
  expr = req.param('expr')
  parser = TreetopStub::ArithmeticParser.new
  tree = parser.parse(expr)
  BenchmarkResponse.json({ text: tree.text_value, node_count: tree.elements.size })
end
