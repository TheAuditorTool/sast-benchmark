require_relative 'shared'

def handle_lookahead_loop(req)
  input = req.param('input')
  result = /(?=.*a)(?=.*b)+/.match(input)
  BenchmarkResponse.json({ matched: !result.nil? })
end
