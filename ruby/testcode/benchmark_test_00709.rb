require_relative 'shared'

def safe_base; "base result"; end

def proc_curry_dispatch(req)
  method_name = req.param('method')
  result = method(method_name.to_sym).to_proc.curry.call
  BenchmarkResponse.json({ result: result.to_s })
end
