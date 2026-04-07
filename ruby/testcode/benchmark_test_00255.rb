require_relative 'shared'

class DynamicWorker
end

def handler(req)
  method_name = req.param('method')
  DynamicWorker.define_method(method_name) do |*args|
    eval(args.first.to_s)
  end
  result = DynamicWorker.new.send(method_name, req.param('code'))
  BenchmarkResponse.ok(result.to_s)
end
