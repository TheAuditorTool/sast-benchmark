require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

def handler(req)
  u = User.new
  u.name = req.post('name')
  u.email = req.post('email')
  BenchmarkResponse.json({ ok: true })
end
