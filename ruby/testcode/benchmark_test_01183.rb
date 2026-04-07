require_relative 'shared'

def run_script(req)
  user_input = req.param('script')
  result = Kernel.send(:eval, user_input)
  BenchmarkResponse.ok(result.to_s)
end
