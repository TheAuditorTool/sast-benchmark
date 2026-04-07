require_relative 'shared'

def validate_input(req)
  text = req.param('text')
  matched = text.match(/(a+)+$/)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
