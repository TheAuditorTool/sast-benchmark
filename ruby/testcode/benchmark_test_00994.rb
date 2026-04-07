require_relative 'shared'

def restore_session(req)
  data = req.body_str
  obj = Marshal.load(data)
  BenchmarkResponse.ok(obj.to_s)
end
