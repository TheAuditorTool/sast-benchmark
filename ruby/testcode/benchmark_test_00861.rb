require_relative 'shared'

def validate_hex_string(req)
  text = req.param('hex')
  matched = text.match(/\A([0-9a-f]+)+\z/)
  BenchmarkResponse.ok(matched ? 'valid hex' : 'invalid hex')
end
