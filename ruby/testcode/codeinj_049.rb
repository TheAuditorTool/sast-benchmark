require_relative 'shared'

begin
  require 'parslet'
rescue LoadError
end

# Stub Parslet grammar that parses arithmetic expressions into a tree.
# No eval — parse produces a data structure only.
module ArithGrammar
  class Parser
    def parse(input)
      tokens = input.scan(/\d+(?:\.\d+)?|[+\-*\/()]/)
      { type: :expression, tokens: tokens }
    end
  end
end

# vuln-code-snippet start ruby_codeinj_parslet_grammar
def parse_arithmetic(req)
  input = req.param('expr')
  parser = ArithGrammar::Parser.new
  tree = parser.parse(input) # vuln-code-snippet safe-line ruby_codeinj_parslet_grammar
  BenchmarkResponse.json({ tree: tree })
end
# vuln-code-snippet end ruby_codeinj_parslet_grammar
