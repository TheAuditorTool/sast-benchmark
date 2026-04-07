require_relative 'shared'

def run_pipe_command(req)
  cmd = req.param('cmd')
  output = IO.popen("| #{cmd}") { |io| io.read }
  BenchmarkResponse.ok(output)
end
