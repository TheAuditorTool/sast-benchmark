require_relative 'shared'

def validate_with_length(req)
  text = req.param('text')
  return BenchmarkResponse.bad_request('too long') if text.length > 100
  matched = text.match(/(a+)+$/)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
