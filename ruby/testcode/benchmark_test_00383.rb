require_relative 'shared'

HANDLER_MAP = {
  'greet' => method(:puts),
}.freeze

def handler(req)
  action = HANDLER_MAP.fetch(req.param('action'), method(:puts))
  action.call('ok')
  BenchmarkResponse.json({ ok: true })
end
