require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

ALLOWED = %w[title body category].freeze

def handler(req)
  validated = req.post_data.slice(*ALLOWED)
  post = User.create(validated)
  BenchmarkResponse.json({ ok: true })
end
