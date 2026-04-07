require_relative 'shared'

def dynamic_action(req)
  body = req.param('body')
  klass = Class.new do
    define_method(:action) { eval(body) }
  end
  result = klass.new.action
  BenchmarkResponse.json({ result: result.to_s })
end
