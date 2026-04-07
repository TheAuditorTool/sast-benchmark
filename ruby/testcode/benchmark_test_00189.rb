require_relative 'shared'

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

def parse_math_expression(req)
  expr = req.param('expr')
  node = ArithParser.parse(expr)
  BenchmarkResponse.json({ value: node.value })
end
