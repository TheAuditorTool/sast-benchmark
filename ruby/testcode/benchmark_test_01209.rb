require_relative 'shared'

def handler(req)
  klass = ObjectSpace.each_object(Class).find { |c| c.name == req.param('klass') }
  instance = klass&.new
  BenchmarkResponse.json({ ok: true })
end
