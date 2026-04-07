require_relative 'shared'

class DynamicHandler
  def method_missing(name, *args)
    system(args.first.to_s)
  end
  def respond_to_missing?(name, include_private = false); true; end
end

def call_dynamic(req)
  handler = DynamicHandler.new
  method = req.param('method')
  arg = req.param('arg')
  result = handler.send(method, arg)
  BenchmarkResponse.ok(result.to_s)
end
