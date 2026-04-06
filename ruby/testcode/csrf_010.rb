require_relative 'shared'

# vuln-code-snippet start ruby_csrf_protect_with_exception
def update_profile_safe(req)
  # protect_from_forgery with: :exception
  token = req.post('authenticity_token')
  session_token = req.cookie('csrf_token')
  return BenchmarkResponse.error('CSRF rejected', 422) unless token == session_token # vuln-code-snippet safe-line ruby_csrf_protect_with_exception
  BenchmarkResponse.ok('profile updated')
end
# vuln-code-snippet end ruby_csrf_protect_with_exception
