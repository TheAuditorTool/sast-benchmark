require_relative 'shared'

ALLOWED_KEYS = %w[name email bio].freeze

def update_sliced(req)
  params = req.post('user') || {}
  safe_attrs = params.is_a?(Hash) ? params.select { |k, _| ALLOWED_KEYS.include?(k.to_s) } : {}
  BenchmarkResponse.ok("updated: #{safe_attrs}")
end
