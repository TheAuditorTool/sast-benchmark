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

def forwardable_handler(req)
  svc    = SafeForwarder.new
  result = svc.read
  BenchmarkResponse.json({ result: result })
end
