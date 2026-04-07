require_relative 'shared'

begin
  require 'parslet'
rescue LoadError
end

module ArithGrammar
  class Parser
    def parse(input)
      tokens = input.scan(/\d+(?:\.\d+)?|[+\-*\/()]/)
      { type: :expression, tokens: tokens }
    end
  end
end

def parse_arithmetic(req)
  input = req.param('expr')
  parser = ArithGrammar::Parser.new
  tree = parser.parse(input)
  BenchmarkResponse.json({ tree: tree })
end
