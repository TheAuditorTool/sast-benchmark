require_relative 'shared'

class DynamicRouter
  def method_missing(name, *args)
    super
  end

  def respond_to_missing?(name, include_private = false)
    super
  end
end

# vuln-code-snippet start ruby_dynmethod_missing_passthrough
def route_action(req)
  obj = DynamicRouter.new
  obj.send(req.param('action')) # vuln-code-snippet vuln-line ruby_dynmethod_missing_passthrough
  BenchmarkResponse.json({ result: "ok" })
end
# vuln-code-snippet end ruby_dynmethod_missing_passthrough
