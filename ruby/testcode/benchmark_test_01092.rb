require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

def handler(req)
  form = OpenStruct.new(req.post_data)
  user = User.create(form.to_h)
  BenchmarkResponse.json({ ok: true })
end
