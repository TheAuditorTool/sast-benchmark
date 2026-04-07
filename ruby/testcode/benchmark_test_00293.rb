require_relative 'shared'

def handle_alternation_loop(req)
  input = req.param('input')
  result = /(red|blue|green)+/.match(input)
  BenchmarkResponse.json({ matched: !result.nil? })
end
