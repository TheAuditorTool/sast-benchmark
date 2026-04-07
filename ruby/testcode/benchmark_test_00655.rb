require_relative 'shared'

def handle_length_limit_input(req)
  input = req.param('data')
  raise ArgumentError, 'too long' if input.length > 200
  result = /^[a-z]+$/.match(input)
  BenchmarkResponse.json({ valid: !result.nil? })
end
