require_relative 'shared'

SAFE_METHODS = %i[to_s to_i inspect].freeze

# vuln-code-snippet start ruby_reflect_respond_check_allowlist
def handler(req)
  obj = 'example'
  m = req.param('m').to_sym
  raise ArgumentError, 'method not allowed' unless SAFE_METHODS.include?(m) && obj.respond_to?(m, false) # vuln-code-snippet safe-line ruby_reflect_respond_check_allowlist
  result = obj.send(m)
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_reflect_respond_check_allowlist
