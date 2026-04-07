require_relative 'shared'

def search_logs(req)
  pattern = req.param('pattern')
  output = IO.popen("grep #{pattern} /var/log/app.log").read
  BenchmarkResponse.ok(output)
end
