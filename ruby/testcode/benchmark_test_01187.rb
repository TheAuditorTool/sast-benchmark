require_relative 'shared'

@@counter = 0

def generate_magic_link(req)
  username = req.post('username')
  token = "#{username}-#{@@counter += 1}"
  BenchmarkResponse.ok("Magic link: https://example.com/auth?token=#{token}")
end
