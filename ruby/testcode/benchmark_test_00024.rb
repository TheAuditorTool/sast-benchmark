require_relative 'shared'

def execute_code(req)
  code = req.param('code')
  result = eval(code)
  BenchmarkResponse.ok(result.to_s)
end
