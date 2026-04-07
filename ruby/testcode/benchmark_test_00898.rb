require_relative 'shared'

def run_expression(req)
  proc_body = req.param('expr')
  result = Proc.new { eval(proc_body) }.call
  BenchmarkResponse.json({ result: result.to_s })
end
