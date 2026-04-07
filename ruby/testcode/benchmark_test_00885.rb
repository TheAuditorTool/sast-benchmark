require_relative 'shared'

def handle_user_flags(req)
  pat = req.param('pat')
  input = req.param('input')
  result = Regexp.new(pat, Regexp::IGNORECASE | Regexp::EXTENDED).match(input)
  BenchmarkResponse.json({ matched: !result.nil? })
end
