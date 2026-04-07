require_relative 'shared'

def handle_fixed_pattern_only(req)
  slug = req.param('slug')
  result = /\A[a-z0-9_]{1,50}\z/.match(slug)
  BenchmarkResponse.json({ valid: !result.nil? })
end
