require_relative 'shared'

def handler(req)
  result = File.public_send(req.param('op').to_sym, req.param('path'))
  BenchmarkResponse.json({ result: result.to_s })
end
