require_relative 'shared'

def redirect_to_url(req)
  input = req.param('url')
  BenchmarkResponse.new(302, '', { 'Location' => input })
end
