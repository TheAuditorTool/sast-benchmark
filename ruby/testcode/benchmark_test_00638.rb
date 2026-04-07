require_relative 'shared'

SAFE = %w[String Integer Float].freeze

def handler(req)
  n = req.param('klass')
  raise ArgumentError, 'not allowed' unless SAFE.include?(n)
  klass = Object.const_get(n)
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
