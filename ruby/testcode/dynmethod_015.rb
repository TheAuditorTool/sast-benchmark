require_relative 'shared'

class Worker; end

# vuln-code-snippet start ruby_dynmethod_class_new_method
def create_worker(req)
  method_name = req.param('method')
  code = req.param('code')
  klass = Class.new(Worker) { define_method(method_name.to_sym) { eval(code) } } # vuln-code-snippet vuln-line ruby_dynmethod_class_new_method
  BenchmarkResponse.ok(klass.new.send(method_name).to_s)
end
# vuln-code-snippet end ruby_dynmethod_class_new_method
