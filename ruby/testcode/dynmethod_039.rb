require_relative 'shared'

begin
  require 'ostruct'
rescue LoadError
end

SAFE_ATTRS_039 = %w[name email].freeze

# vuln-code-snippet start ruby_dynmethod_respond_allowlist
def respond_allowlist(req)
  obj = OpenStruct.new(name: 'alice', email: 'alice@example.com')
  m   = req.param('attr')
  raise ArgumentError, 'forbidden' unless SAFE_ATTRS_039.include?(m) && obj.respond_to?(m, false) # vuln-code-snippet safe-line ruby_dynmethod_respond_allowlist
  result = obj.send(m)
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_respond_allowlist
