require_relative 'shared'

ALLOWED_HOSTS = %w[app.example.com dashboard.example.com].freeze

def post_login(req)
  username = req.post('username')
  password = req.post('password')
  next_url = req.param('next')
  parsed = URI.parse(next_url) rescue nil
  destination = (parsed && ALLOWED_HOSTS.include?(parsed.host)) ? next_url : '/dashboard'
  BenchmarkResponse.redirect(destination)
end
