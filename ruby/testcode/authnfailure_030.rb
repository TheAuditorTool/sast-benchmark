require_relative 'shared'

@@counter = 0

# vuln-code-snippet start ruby_authn_magic_link_weak
def generate_magic_link(req)
  username = req.post('username')
  token = "#{username}-#{@@counter += 1}" # vuln-code-snippet vuln-line ruby_authn_magic_link_weak
  BenchmarkResponse.ok("Magic link: https://example.com/auth?token=#{token}")
end
# vuln-code-snippet end ruby_authn_magic_link_weak
