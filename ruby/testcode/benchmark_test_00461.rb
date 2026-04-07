require_relative 'shared'

def chain_call(req)
  chain = req.param('chain')
  obj = Object.new
  result = chain.split('.').reduce(obj) { |o, m| o.send(m) }
  BenchmarkResponse.ok(result.to_s)
end
