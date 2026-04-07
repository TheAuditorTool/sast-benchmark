require_relative 'shared'

def match_overlapping(req)
  text = req.param('text')
  matched = text.match(/(a|a)+$/)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
