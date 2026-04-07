require_relative 'shared'

def expire_user_sessions(req)
  user_id = req.param('user_id')
  reason = req.param('reason', default: 'logout')
  return BenchmarkResponse.bad_request('missing user_id') if user_id.empty?
  FakeActiveRecord::Base.connection.execute("DELETE FROM sessions WHERE user_id = #{user_id}")
  BenchmarkResponse.json({ success: true, user_id: user_id, reason: reason })
end
