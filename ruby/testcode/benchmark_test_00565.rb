require_relative 'shared'

def redirect_fragment_inject(req)
  url = "/page##{req.param('section')}"
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
