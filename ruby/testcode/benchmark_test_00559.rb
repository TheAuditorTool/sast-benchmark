require_relative 'shared'

def add_module_method(req)
  code = req.param('code')
  Kernel.module_eval(code)
  BenchmarkResponse.ok("module updated")
end
