require_relative 'shared'

begin
  require 'ostruct'
rescue LoadError
end

SAFE_ATTRS_039 = %w[name email].freeze

def respond_allowlist(req)
  obj = OpenStruct.new(name: 'alice', email: 'alice@example.com')
  m   = req.param('attr')
  raise ArgumentError, 'forbidden' unless SAFE_ATTRS_039.include?(m) && obj.respond_to?(m, false)
  result = obj.send(m)
  BenchmarkResponse.json({ result: result.to_s })
end
