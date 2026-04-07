require_relative 'shared'

ACTIONS = { 'start' => :do_start, 'stop' => :do_stop }.freeze

def do_start(data); "started: #{data}"; end
def do_stop(data); "stopped: #{data}"; end

def dispatch_frozen(req)
  action = req.param('action')
  sym = ACTIONS[action]
  return BenchmarkResponse.bad_request('unknown') unless sym
  BenchmarkResponse.ok(send(sym, req.param('data')))
end
