require_relative 'shared'

def handle_backreference_complex(req)
  input = req.param('input')
  result = /((\w+)\s+\2)+/.match(input)
  BenchmarkResponse.json({ matched: !result.nil? })
end
