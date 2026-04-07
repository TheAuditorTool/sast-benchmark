require_relative 'shared'

begin
  require 'dentaku'
rescue LoadError
end

def calculate_expression(req)
  expr = req.param('expr')
  calculator = Dentaku::Calculator.new
  result = calculator.evaluate(expr)
  BenchmarkResponse.json({ result: result })
end
