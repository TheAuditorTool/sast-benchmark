require_relative 'shared'

def redirect_login_fixed(req)
  BenchmarkResponse.new(302, { 'Location' => '/dashboard' }, '')
end
