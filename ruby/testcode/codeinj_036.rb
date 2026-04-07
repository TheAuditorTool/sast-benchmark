require_relative 'shared'

begin
  require 'dentaku'
rescue LoadError
end

# vuln-code-snippet start ruby_codeinj_dentaku_calc
def calculate_expression(req)
  expr = req.param('expr')
  calculator = Dentaku::Calculator.new
  result = calculator.evaluate(expr) # vuln-code-snippet safe-line ruby_codeinj_dentaku_calc
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_codeinj_dentaku_calc
