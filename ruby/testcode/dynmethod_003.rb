require_relative 'shared'

class EvalProxy
  def method_missing(name, *args)
    eval(args.first.to_s)
  end

  def respond_to_missing?(_name, _include_private = false)
    true
  end
end

# vuln-code-snippet start ruby_dynmethod_method_missing
def dynmethod_method_missing(req)
  proxy = EvalProxy.new
  result = proxy.send(req.param('action'), req.param('expr')) # vuln-code-snippet vuln-line ruby_dynmethod_method_missing
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_dynmethod_method_missing
