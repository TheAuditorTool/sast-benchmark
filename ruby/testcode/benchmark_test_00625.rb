require_relative 'shared'

def append_log_entry(req)
  logfile = req.param('logfile')
  content = req.post('content')
  File.write("/logs/#{logfile}", content)
  BenchmarkResponse.ok("Log entry written")
end
