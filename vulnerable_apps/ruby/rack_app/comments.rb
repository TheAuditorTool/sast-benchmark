require_relative '../../testcode/shared'

# rack_app - Comments and redirects

# vuln-code-snippet start rk_headerinj_redirect
def redirect_after_comment(env)
  req = Rack::Request.new(env)
  return_url = req.params['return_to']
  [302, { 'Content-Type' => 'text/plain', 'Location' => return_url }, ['']] # vuln-code-snippet vuln-line rk_headerinj_redirect
end
# vuln-code-snippet end rk_headerinj_redirect

# vuln-code-snippet start rk_headerinj_safe
def redirect_after_comment_safe(env)
  req = Rack::Request.new(env)
  return_url = req.params['return_to'].to_s.gsub(/[\r\n]/, '') # vuln-code-snippet safe-line rk_headerinj_safe
  [302, { 'Content-Type' => 'text/plain', 'Location' => return_url }, ['']]
end
# vuln-code-snippet end rk_headerinj_safe
