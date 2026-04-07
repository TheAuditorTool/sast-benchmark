require_relative 'shared'

def dns_lookup(req)
  domain = req.param('domain')
  result = %x(dig
  BenchmarkResponse.ok(result)
end
