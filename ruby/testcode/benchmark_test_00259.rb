require_relative 'shared'

def redirect_subdomain_bypass(req)
  url = "https://#{req.param('sub')}.example.com/home"
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
