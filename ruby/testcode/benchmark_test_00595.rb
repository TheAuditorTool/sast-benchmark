require_relative 'shared'

def log_to_file(req)
  message = req.param('message')
  File.open('application.log', 'a') { |f| f.puts(message) }
  BenchmarkResponse.ok('logged')
end
