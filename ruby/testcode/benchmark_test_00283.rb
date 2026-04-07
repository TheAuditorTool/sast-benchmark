require_relative 'shared'

def handle_re2_gem(req)
  pat = req.param('pat')
  input = req.param('input')
  result = RE2::Regexp.new(pat).match(input)
  BenchmarkResponse.json({ matched: !result.nil? })
end
