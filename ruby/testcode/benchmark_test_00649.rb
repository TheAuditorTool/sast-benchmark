require_relative 'shared'

def validate_numeric(req)
  text = req.param('number')
  matched = text.match(/\A\d+\z/)
  BenchmarkResponse.ok(matched ? 'valid number' : 'invalid')
end
