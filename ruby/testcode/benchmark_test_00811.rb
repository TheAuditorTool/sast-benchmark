require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

REQUIRED = %w[email password].freeze

def handler(req)
  raise 'missing required params' unless REQUIRED.all? { |k| req.post_data.key?(k) }
  user = User.create(req.post_data.slice(*REQUIRED))
  BenchmarkResponse.json({ ok: true })
end
