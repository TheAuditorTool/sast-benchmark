require_relative 'shared'

def validate_sentence(req)
  text = req.param('text')
  matched = text.match(/(\w+\s?)+$/)
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
