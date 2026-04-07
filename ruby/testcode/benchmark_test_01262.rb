require_relative 'shared'

def handle_atomic_group_match(req)
  input = req.param('input')
  result = /(?>ab|a)bc/.match(input)
  BenchmarkResponse.json({ matched: !result.nil? })
end
