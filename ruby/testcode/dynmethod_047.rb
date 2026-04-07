require_relative 'shared'

class FixedTarget047
  def read;  "reading";  end
  def write; "writing";  end
  def list;  "listing";  end
end

METHODS_047 = %i[read write list].freeze

# vuln-code-snippet start ruby_dynmethod_method_fixed_list
def method_fixed_list(req)
  m      = req.param('m').to_sym
  target = FixedTarget047.new
  raise ArgumentError, 'not in list' unless METHODS_047.include?(m) # vuln-code-snippet safe-line ruby_dynmethod_method_fixed_list
  result = target.send(m)
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_method_fixed_list
