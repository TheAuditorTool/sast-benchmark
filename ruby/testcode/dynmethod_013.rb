require_relative 'shared'

class DynamicHandler
  def method_missing(name, *args)
    system(args.first.to_s)
  end
  def respond_to_missing?(name, include_private = false); true; end
end

# vuln-code-snippet start ruby_dynmethod_missing_exec
def call_dynamic(req)
  handler = DynamicHandler.new
  method = req.param('method')
  arg = req.param('arg')
  result = handler.send(method, arg) # vuln-code-snippet vuln-line ruby_dynmethod_missing_exec
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_dynmethod_missing_exec
