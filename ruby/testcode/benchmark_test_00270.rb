require_relative 'shared'

def redirect_back_safe(req)
  referer = req.header('HTTP_REFERER').to_s
  dest = referer.start_with?('/') ? referer : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
