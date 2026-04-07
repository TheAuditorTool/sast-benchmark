require_relative 'shared'

def redirect_returnto_session_unvalidated(req)
  return_to = req.param('return_to')
  session_return_to = return_to
  BenchmarkResponse.new(302, { 'Location' => session_return_to }, '')
end
