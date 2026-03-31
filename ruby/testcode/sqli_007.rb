require_relative 'shared'

# vuln-code-snippet start ruby_sqli_connection_execute
def expire_user_sessions(req)
  user_id = req.param('user_id')
  reason = req.param('reason', default: 'logout')
  return BenchmarkResponse.bad_request('missing user_id') if user_id.empty?
  FakeActiveRecord::Base.connection.execute("DELETE FROM sessions WHERE user_id = #{user_id}")  # vuln-code-snippet vuln-line ruby_sqli_connection_execute
  BenchmarkResponse.json({ success: true, user_id: user_id, reason: reason })
end
# vuln-code-snippet end ruby_sqli_connection_execute
