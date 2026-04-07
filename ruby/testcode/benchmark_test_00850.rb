require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

def handler(req)
  fields = req.param('fields').split(',').map(&:to_sym)
  user = User.create(req.post_data.slice(*fields))
  BenchmarkResponse.json({ ok: true })
end
