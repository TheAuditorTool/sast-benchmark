require_relative 'shared'

def handle_possessive_anchor(req)
  input = req.param('input')
  result = /\A\w++\z/.match(input)
  BenchmarkResponse.json({ valid: !result.nil? })
end
