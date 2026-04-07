require_relative 'shared'

def redirect_callback_param(req)
  url = req.param('callback')
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
