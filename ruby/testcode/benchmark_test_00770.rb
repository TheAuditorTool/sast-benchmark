require_relative 'shared'

def log_stderr(req)
  user_input = req.param('message')
  $stderr.write("Error: #{user_input}\n")
  BenchmarkResponse.ok('error logged')
end
