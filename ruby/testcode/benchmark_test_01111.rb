require_relative 'shared'

def redirect_relative_check(req)
  url = req.param('next')
  raise 'invalid' unless url.start_with?('/') && !url.start_with?('//')
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
