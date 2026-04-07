require_relative 'shared'

VALID_TYPES = %w[text/html application/json text/plain].freeze

def content_type_safe(req)
  ctype = req.param('type')
  return BenchmarkResponse.bad_request('invalid') unless VALID_TYPES.include?(ctype)
  BenchmarkResponse.new(200, 'ok', { 'Content-Type' => ctype })
end
