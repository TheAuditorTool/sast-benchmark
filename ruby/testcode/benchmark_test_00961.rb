require_relative 'shared'

def redirect_referer_check(req)
  referer = req.header('HTTP_REFERER').to_s
  dest = referer.include?('example.com') ? referer : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
