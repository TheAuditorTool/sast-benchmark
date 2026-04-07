require_relative 'shared'

require 'delegate'

class BackendService050
  def specific_method; "backend result"; end
end

class SafeWrapper050 < SimpleDelegator
  def allowed_method
    __getobj__.specific_method
  end
end

def simple_delegate_handler(req)
  wrapper = SafeWrapper050.new(BackendService050.new)
  result  = wrapper.allowed_method
  BenchmarkResponse.json({ result: result })
end
