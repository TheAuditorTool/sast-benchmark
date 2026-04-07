require_relative 'shared'

def binding_dispatch(req)
  x = 42
  var_name = req.param('var_name').to_sym
  result = binding.local_variable_get(var_name)
  BenchmarkResponse.json({ result: result.to_s })
end
