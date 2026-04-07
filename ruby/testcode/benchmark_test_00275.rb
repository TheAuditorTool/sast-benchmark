require_relative 'shared'

HANDLERS = {
  'greet' => -> { 'hello' },
  'ping'  => -> { 'pong' },
  'time'  => -> { Time.now.utc.iso8601 },
  'echo'  => -> { 'echo' }
}.freeze

def dispatch_action(req)
  action = req.param('action')
  handler = HANDLERS[action]
  result = handler&.call
  return BenchmarkResponse.bad_request('unknown action') if result.nil?
  BenchmarkResponse.json({ result: result })
end
