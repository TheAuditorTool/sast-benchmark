require_relative 'shared'

def match_atomic(req)
  text = req.param('text')
  matched = text.match(/(?>a+)b/)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
