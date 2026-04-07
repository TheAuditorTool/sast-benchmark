require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

def lookup_role_from_db(user_id)
  'user'
end

def handler(req)
  role = lookup_role_from_db(req.cookie('user_id'))
  user = User.create(req.post_data.except('role', 'admin'))
  user.update(role: role)
  BenchmarkResponse.json({ ok: true })
end
