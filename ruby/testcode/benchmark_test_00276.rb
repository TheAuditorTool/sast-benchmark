require_relative 'shared'

def handler(req)
  output = req.param('fmt') % { name: 'world', value: 42 }
  BenchmarkResponse.html(output)
end
