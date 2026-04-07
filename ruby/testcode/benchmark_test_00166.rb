require_relative 'shared'

SLUG_PATTERN = /\A[a-z0-9\-]+\z/

def validate_slug(req)
  input = req.param('slug')
  truncated = input[0, 1000]
  matched = SLUG_PATTERN.match(truncated)
  BenchmarkResponse.ok(matched ? 'valid slug' : 'invalid slug')
end
