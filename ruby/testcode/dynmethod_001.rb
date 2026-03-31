require_relative 'shared'

class DynamicWorker
end

# vuln-code-snippet start ruby_dynmethod_define_taint
def dynmethod_define_taint(req)
  method_name = req.param('method')
  DynamicWorker.define_method(method_name) do |*args|
    eval(args.first.to_s) # vuln-code-snippet vuln-line ruby_dynmethod_define_taint
  end
  result = DynamicWorker.new.send(method_name, req.param('code'))
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_dynmethod_define_taint
