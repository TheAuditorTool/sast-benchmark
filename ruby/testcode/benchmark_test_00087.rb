require_relative 'shared'

class Worker; end

def create_worker(req)
  method_name = req.param('method')
  code = req.param('code')
  klass = Class.new(Worker) { define_method(method_name.to_sym) { eval(code) } }
  BenchmarkResponse.ok(klass.new.send(method_name).to_s)
end
