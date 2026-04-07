require_relative 'shared'

def load_extension(req)
  code = req.param('code')
  mod = Module.new do
    module_function
    eval(code)
  end
  mod.extend(Object)
  BenchmarkResponse.json({ status: 'loaded' })
end
