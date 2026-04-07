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

# vuln-code-snippet start ruby_dynmethod_simple_delegate_fixed
def simple_delegate_handler(req)
  wrapper = SafeWrapper050.new(BackendService050.new)
  result  = wrapper.allowed_method # vuln-code-snippet safe-line ruby_dynmethod_simple_delegate_fixed
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_simple_delegate_fixed
