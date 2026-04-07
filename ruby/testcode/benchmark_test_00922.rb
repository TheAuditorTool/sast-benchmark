require_relative 'shared'

def extend_module(req)
  tainted_code = req.param('code')
  result = Module.new.module_exec { eval(tainted_code) }
  BenchmarkResponse.json({ result: result.to_s })
end
