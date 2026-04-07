require_relative 'shared'

def validate_input_timeout(req)
  text = req.param('text')
  safe_re = Regexp.new('(a+)+$', timeout: 1.0)
  matched = safe_re.match(text)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
