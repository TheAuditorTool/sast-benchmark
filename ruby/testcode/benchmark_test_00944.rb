require_relative 'shared'

SAFE_NAMES = %w[UserReport AdminReport].freeze

def handler(req)
  n = req.param('name')
  raise ArgumentError, 'name not allowed' unless SAFE_NAMES.include?(n)
  klass = Object.const_get("Reports::#{n}")
  BenchmarkResponse.json({ ok: true })
end
