require_relative 'shared'

def handler(req)
  result = Module.new.extend(Kernel).send(req.param('method').to_sym)
  BenchmarkResponse.json({ result: result.to_s })
end
