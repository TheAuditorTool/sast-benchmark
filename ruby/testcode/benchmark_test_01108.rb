require_relative 'shared'

class EvalProxy
  def method_missing(name, *args)
    eval(args.first.to_s)
  end

  def respond_to_missing?(_name, _include_private = false)
    true
  end
end

def handler(req)
  proxy = EvalProxy.new
  result = proxy.send(req.param('action'), req.param('expr'))
  BenchmarkResponse.ok(result.to_s)
end
