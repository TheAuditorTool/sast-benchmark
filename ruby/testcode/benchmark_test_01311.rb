require_relative 'shared'

def restore_session(req)
  payload = req.cookie('session_data')
  session = Marshal.load(Base64.decode64(payload))
  BenchmarkResponse.json({ user: session[:user], role: session[:role] })
end
