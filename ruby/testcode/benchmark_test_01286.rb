require_relative 'shared'

def redirect_via_header(req)
  url = req.param('url')
  BenchmarkResponse.new(302, '', { 'Location' => url })
end
