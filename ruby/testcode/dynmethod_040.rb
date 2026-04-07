require_relative 'shared'

require 'forwardable'

class FixedBackend
  def read; "reading from backend"; end
end

class SafeForwarder
  extend Forwardable
  def_delegator :@backend, :read

  def initialize
    @backend = FixedBackend.new
  end
end

# vuln-code-snippet start ruby_dynmethod_forwardable_fixed
def forwardable_handler(req)
  svc    = SafeForwarder.new
  result = svc.read # vuln-code-snippet safe-line ruby_dynmethod_forwardable_fixed
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_forwardable_fixed
