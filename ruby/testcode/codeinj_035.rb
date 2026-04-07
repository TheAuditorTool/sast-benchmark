require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_catch_throw_eval
def evaluate_and_catch(req)
  expr = req.param('expr')
  result = catch(:result) { throw :result, eval(expr) } # vuln-code-snippet vuln-line ruby_codeinj_catch_throw_eval
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_catch_throw_eval
