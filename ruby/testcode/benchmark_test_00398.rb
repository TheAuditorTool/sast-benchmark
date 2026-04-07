require_relative 'shared'

def handler(req)
  mod = Object.ancestors.find { |m| m.name == req.param('mod') }
  result = mod&.send(req.param('method').to_sym)
  BenchmarkResponse.json({ result: result.to_s })
end
