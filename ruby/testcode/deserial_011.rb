require_relative 'shared'
require 'base64'

# vuln-code-snippet start ruby_deser_marshal_base64
def resume_session(req)
  encoded = req.post('session')
  obj = Marshal.load(Base64.decode64(encoded)) # vuln-code-snippet vuln-line ruby_deser_marshal_base64
  user_id = obj[:user_id].to_s
  BenchmarkResponse.ok("session for user #{user_id}")
end
# vuln-code-snippet end ruby_deser_marshal_base64
