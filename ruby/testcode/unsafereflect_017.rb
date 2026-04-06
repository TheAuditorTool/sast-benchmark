require_relative 'shared'

# vuln-code-snippet start ruby_reflect_send_chain
def chain_call(req)
  chain = req.param('chain')
  obj = Object.new
  result = chain.split('.').reduce(obj) { |o, m| o.send(m) } # vuln-code-snippet vuln-line ruby_reflect_send_chain
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_reflect_send_chain
