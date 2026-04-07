require_relative 'shared'

def handle_length_then_simple(req)
  val = req.param('code')
  raise ArgumentError if val.length > 100
  result = val.match?(/\A\d+\z/)
  BenchmarkResponse.json({ valid: result })
end
