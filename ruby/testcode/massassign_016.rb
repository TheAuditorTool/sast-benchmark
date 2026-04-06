require_relative 'shared'

ALLOWED_KEYS = %w[name email bio].freeze

# vuln-code-snippet start ruby_massassign_hash_slice
def update_sliced(req)
  params = req.post('user') || {}
  safe_attrs = params.is_a?(Hash) ? params.select { |k, _| ALLOWED_KEYS.include?(k.to_s) } : {} # vuln-code-snippet safe-line ruby_massassign_hash_slice
  BenchmarkResponse.ok("updated: #{safe_attrs}")
end
# vuln-code-snippet end ruby_massassign_hash_slice
