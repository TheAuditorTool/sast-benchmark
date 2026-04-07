require_relative 'shared'

def set_tracking_cookie(req)
  name = req.param('name')
  value = 'tracker_value_constant'
  response = BenchmarkResponse.ok('cookie set')
  response.headers['Set-Cookie'] = "#{name}=#{value}"
  response
end
