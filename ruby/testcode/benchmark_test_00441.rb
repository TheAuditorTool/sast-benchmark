require_relative 'shared'

def patch_object(req)
  code = req.param('code')
  Object.class_eval(code)
  BenchmarkResponse.json({ status: 'applied' })
end
