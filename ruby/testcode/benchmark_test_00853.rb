require_relative 'shared'

def redirect_scheme_inject(req)
  url = "#{req.param('scheme')}://safe.example.com/home"
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
