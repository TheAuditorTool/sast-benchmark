require_relative 'shared'

def redirect_allow_other_host(req)
  url = req.param('url')
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
