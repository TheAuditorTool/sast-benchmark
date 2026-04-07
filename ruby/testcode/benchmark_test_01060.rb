require_relative 'shared'

def handler(req)
  autoload(:User, 'models/user')
  BenchmarkResponse.json({ ok: true })
end
