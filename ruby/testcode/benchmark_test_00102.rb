require_relative 'shared'

def redirect_sinatra_indirect(req)
  BenchmarkResponse.new(302, { 'Location' => req.param('destination') }, '')
end
