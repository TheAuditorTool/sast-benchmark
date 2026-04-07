require_relative 'shared'

SAFE_METHODS = %i[to_s to_i inspect].freeze

def handler(req)
  obj = 'example'
  m = req.param('m').to_sym
  raise ArgumentError, 'method not allowed' unless SAFE_METHODS.include?(m) && obj.respond_to?(m, false)
  result = obj.send(m)
  BenchmarkResponse.json({ result: result.to_s })
end
