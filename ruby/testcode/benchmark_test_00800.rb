require_relative 'shared'

def process_upload(req)
  raw_name = req.param('filename')
  safe_name = File.basename(raw_name)
  unless safe_name =~ /\A[a-zA-Z0-9._-]+\z/
    return BenchmarkResponse.bad_request("invalid filename")
  end
  result = `file /uploads/
  BenchmarkResponse.ok(result)
end
