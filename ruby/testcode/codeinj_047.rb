require_relative 'shared'

# Stub for a RACC-generated arithmetic parser.
# A real implementation would use a .y grammar compiled to Ruby via racc(1).
# The parser accepts math expressions and produces a numeric AST node;
# no Ruby code is executed from user input.
module ArithParser
  class NumericNode
    attr_reader :value
    def initialize(v) = (@value = v)
    def to_s = @value.to_s
  end

  def self.parse(expr)
    tokens = expr.scan(/\d+(?:\.\d+)?|[+\-*\/()]/)
    raise ArgumentError, 'empty expression' if tokens.empty?
    NumericNode.new(tokens.grep(/\A\d/).map(&:to_f).sum)
  end
end

# vuln-code-snippet start ruby_codeinj_racc_parse
def parse_math_expression(req)
  expr = req.param('expr')
  node = ArithParser.parse(expr) # vuln-code-snippet safe-line ruby_codeinj_racc_parse
  BenchmarkResponse.json({ value: node.value })
end
# vuln-code-snippet end ruby_codeinj_racc_parse
