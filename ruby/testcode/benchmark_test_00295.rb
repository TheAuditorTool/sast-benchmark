require_relative 'shared'

class DynamicRouter
  def method_missing(name, *args)
    super
  end

  def respond_to_missing?(name, include_private = false)
    super
  end
end

def route_action(req)
  obj = DynamicRouter.new
  obj.send(req.param('action'))
  BenchmarkResponse.json({ result: "ok" })
end
