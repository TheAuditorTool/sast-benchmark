require_relative 'shared'

def eval_in_binding(req)
  template = req.param('expr')
  result = binding.eval(template)
  BenchmarkResponse.ok(result.to_s)
end
