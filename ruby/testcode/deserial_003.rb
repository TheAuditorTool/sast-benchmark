require_relative 'shared'
require 'base64'

# vuln-code-snippet start ruby_deser_marshal_restore
def load_cookie_session(req)
  cookie_data = req.cookie('session')
  obj = Marshal.restore(Base64.decode64(cookie_data)) # vuln-code-snippet vuln-line ruby_deser_marshal_restore
  BenchmarkResponse.ok(obj.to_s)
end
# vuln-code-snippet end ruby_deser_marshal_restore
