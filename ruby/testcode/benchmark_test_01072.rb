require_relative 'shared'

def handle_regex_construct(req)
  pattern = req.param('pat')
  input = req.param('input')
  result = Regexp.new(pattern).match(input)
  BenchmarkResponse.json({ matched: !result.nil? })
end
