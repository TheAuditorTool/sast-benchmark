require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

def handler(req)
  name  = req.post('name')
  email = req.post('email')
  user  = User.new(name: name, email: email)
  BenchmarkResponse.json({ ok: true })
end
