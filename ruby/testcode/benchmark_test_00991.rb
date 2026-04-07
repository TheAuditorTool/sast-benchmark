require_relative 'shared'

def match_possessive(req)
  text = req.param('text')
  matched = text.match(/\A(a++)b\z/)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
