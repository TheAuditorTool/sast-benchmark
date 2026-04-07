require_relative 'shared'

def validate_hex_id(req)
  input = req.param('id')
  matched = input.match(/\A[a-f0-9]{8}\z/)
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
