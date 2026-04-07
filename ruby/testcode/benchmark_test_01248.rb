require_relative 'shared'

ALLOWED_METHODS_036 = %i[to_s to_i length upcase].freeze

def allowlist_dispatch(req)
  m = req.param('method').to_sym
  raise ArgumentError, 'forbidden' unless ALLOWED_METHODS_036.include?(m)
  result = "hello".send(m)
  BenchmarkResponse.json({ result: result.to_s })
end
