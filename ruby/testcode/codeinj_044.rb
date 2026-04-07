require_relative 'shared'

HANDLERS = {
  'greet' => -> { 'hello' },
  'ping'  => -> { 'pong' },
  'time'  => -> { Time.now.utc.iso8601 },
  'echo'  => -> { 'echo' }
}.freeze

# vuln-code-snippet start ruby_codeinj_const_proc_dispatch
def dispatch_action(req)
  action = req.param('action')
  handler = HANDLERS[action]
  result = handler&.call # vuln-code-snippet safe-line ruby_codeinj_const_proc_dispatch
  return BenchmarkResponse.bad_request('unknown action') if result.nil?
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_codeinj_const_proc_dispatch
