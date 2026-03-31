require_relative 'shared'

# vuln-code-snippet start ruby_deser_marshal_load
def restore_session(req)
  data = req.body_str
  obj = Marshal.load(data) # vuln-code-snippet vuln-line ruby_deser_marshal_load
  BenchmarkResponse.ok(obj.to_s)
end
# vuln-code-snippet end ruby_deser_marshal_load
