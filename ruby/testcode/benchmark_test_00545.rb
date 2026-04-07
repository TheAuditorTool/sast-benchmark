require_relative 'shared'

def fetch_log_file(req)
  filename = req.param('filename')
  unless filename =~ /\A[a-zA-Z0-9._-]+\z/
    return BenchmarkResponse.bad_request("invalid filename")
  end
  result = `cat /var/log/
  BenchmarkResponse.ok(result)
end
