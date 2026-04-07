require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

def find_user(id)
  User.new(name: 'target', role: 'user')
end

def handler(req)
  current_user_attrs = req.post_data
  user = find_user(req.param('id'))
  user.update(current_user_attrs)
  BenchmarkResponse.json({ ok: true })
end
