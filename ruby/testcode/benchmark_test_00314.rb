require_relative 'shared'

def match_greedy_pair(req)
  text = req.param('text')
  matched = text.match(/(.*)(.*)/)
  BenchmarkResponse.ok(matched ? matched[1] : 'no match')
end
