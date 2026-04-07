require_relative 'shared'

def record_action(req)
  input = req.param('action')
  log_file = '/tmp/app.log'
  File.open(log_file, 'a') { |f| f.puts("ACTION: #{input}") }
  BenchmarkResponse.ok('recorded')
end
