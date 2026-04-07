require_relative 'shared'

class MyService
  def process; 'done'; end
end

# vuln-code-snippet start ruby_reflect_method_own_class
def handler(req)
  m = req.param('m').to_sym
  raise ArgumentError, 'method not allowed' unless MyService.public_method_defined?(m) # vuln-code-snippet safe-line ruby_reflect_method_own_class
  result = MyService.new.send(m)
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_reflect_method_own_class
