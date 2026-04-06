require_relative 'shared'

class TaskRunner
  def run(cmd); cmd; end
  def status; 'idle'; end
end

# vuln-code-snippet start ruby_dynmethod_send_tainted
def dispatch_action(req)
  action = req.param('action')
  data = req.param('data')
  runner = TaskRunner.new
  result = runner.send(action, data) # vuln-code-snippet vuln-line ruby_dynmethod_send_tainted
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_dynmethod_send_tainted
