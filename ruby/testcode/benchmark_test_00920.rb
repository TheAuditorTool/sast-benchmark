require_relative 'shared'
require 'bigdecimal'

module ShuntingYard
  OPS = { '+' => [1, :+], '-' => [1, :-], '*' => [2, :*], '/' => [2, :/] }.freeze

  def self.calculate(expr)
    tokens = expr.scan(/\d+(?:\.\d+)?|[+\-*\/()]/)
    output = []
    ops = []

    tokens.each do |tok|
      if tok =~ /\A\d/
        output << BigDecimal(tok)
      elsif OPS.key?(tok)
        while (top = ops.last) && OPS.key?(top) && OPS[top][0] >= OPS[tok][0]
          apply(output, ops.pop)
        end
        ops << tok
      elsif tok == '('
        ops << tok
      elsif tok == ')'
        apply(output, ops.pop) while ops.last != '('
        ops.pop
      end
    end
    apply(output, ops.pop) until ops.empty?
    output.last
  end

  def self.apply(output, op)
    b, a = output.pop, output.pop
    output << a.send(OPS[op][1], b)
  end
end

def calculate_infix(req)
  expr = req.param('expr')
  return BenchmarkResponse.bad_request('invalid chars') unless expr.match?(/\A[\d\s+\-*\/().]+\z/)
  result = ShuntingYard.calculate(expr)
  BenchmarkResponse.json({ result: result.to_s('F') })
end
