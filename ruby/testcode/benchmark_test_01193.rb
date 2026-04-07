require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

def handler(req)
  schema = { 'name' => String, 'email' => String }
  attrs = req.post_data.select { |k, v| schema.key?(k) && v.is_a?(schema[k]) }
  user = User.create(attrs)
  BenchmarkResponse.json({ ok: true })
end
