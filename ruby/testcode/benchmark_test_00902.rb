require_relative 'shared'

def handle_string_match_tainted(req)
  input = req.param('input')
  result = input.match(/^(a+)+$/)
  BenchmarkResponse.json({ matched: !result.nil? })
end
