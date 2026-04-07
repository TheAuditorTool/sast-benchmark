require_relative 'shared'

def match_escaped_pattern(req)
  pattern = req.param('pattern')
  text = req.param('text')
  safe_pattern = Regexp.escape(pattern)
  re = Regexp.new(safe_pattern)
  matched = re.match(text)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
