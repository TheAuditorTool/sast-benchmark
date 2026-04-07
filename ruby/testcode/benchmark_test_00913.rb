require_relative 'shared'

class TaskRunner
  def run(cmd); cmd; end
  def status; 'idle'; end
end

def dispatch_action(req)
  action = req.param('action')
  data = req.param('data')
  runner = TaskRunner.new
  result = runner.send(action, data)
  BenchmarkResponse.ok(result.to_s)
end
