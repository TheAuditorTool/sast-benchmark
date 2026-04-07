require_relative 'shared'

def match_user_pattern(req)
  pattern = req.param('pattern')
  text = req.param('text')
  re = Regexp.new(pattern)
  matched = re.match(text)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
