require_relative 'shared'

def handler(req)
  klass = req.param('class_name').split('::').inject(Object) { |m, n| m.const_get(n) }
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
