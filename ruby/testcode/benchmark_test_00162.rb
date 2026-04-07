require_relative 'shared'

def redirect_host_concat2(req)
  url = "https://#{req.param('host')}/dashboard"
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
