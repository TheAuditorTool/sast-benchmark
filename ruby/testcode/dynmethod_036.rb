require_relative 'shared'

ALLOWED_METHODS_036 = %i[to_s to_i length upcase].freeze

# vuln-code-snippet start ruby_dynmethod_allowlist_dispatch
def allowlist_dispatch(req)
  m = req.param('method').to_sym
  raise ArgumentError, 'forbidden' unless ALLOWED_METHODS_036.include?(m) # vuln-code-snippet safe-line ruby_dynmethod_allowlist_dispatch
  result = "hello".send(m)
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_allowlist_dispatch
