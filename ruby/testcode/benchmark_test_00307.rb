require_relative 'shared'

def handle_no_user_pattern(req)
  slug = req.param('slug')
  valid = /\A[a-z0-9\-]+\z/.match?(slug)
  BenchmarkResponse.json({ valid: valid })
end
