require_relative 'shared'

def handler(req)
  $LOAD_PATH << req.param('dir')
  require req.param('lib')
  BenchmarkResponse.json({ ok: true })
end
