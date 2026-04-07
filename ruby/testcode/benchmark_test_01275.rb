require_relative 'shared'

def execute_snippet(req)
  frozen_input = req.param('code').freeze
  unfrozen = frozen_input.dup
  result = eval(unfrozen)
  BenchmarkResponse.json({ result: result.to_s })
end
