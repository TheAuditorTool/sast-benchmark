require_relative 'shared'

def redirect_no_open_redirect(req)
  BenchmarkResponse.new(302, { 'Location' => '/thank-you' }, '')
end
