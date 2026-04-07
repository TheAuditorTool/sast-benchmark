require_relative 'shared'

def redirect_session_validated(req)
  url = req.param('return_to')
  raise unless url =~ /\A\/[a-z0-9\/\-_]*\z/
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
