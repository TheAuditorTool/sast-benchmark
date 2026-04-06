require_relative 'shared'

ACTIONS = { 'start' => :do_start, 'stop' => :do_stop }.freeze

def do_start(data); "started: #{data}"; end
def do_stop(data); "stopped: #{data}"; end

# vuln-code-snippet start ruby_dynmethod_frozen_methods
def dispatch_frozen(req)
  action = req.param('action')
  sym = ACTIONS[action] # vuln-code-snippet safe-line ruby_dynmethod_frozen_methods
  return BenchmarkResponse.bad_request('unknown') unless sym
  BenchmarkResponse.ok(send(sym, req.param('data')))
end
# vuln-code-snippet end ruby_dynmethod_frozen_methods
