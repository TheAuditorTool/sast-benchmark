require_relative 'shared'

class User
  attr_accessor :id, :name, :email

  def initialize(id, name, email)
    @id = id
    @name = name
    @email = email
  end

  def self.find(id)
    new(id, "user_#{id}", "user#{id}@example.com")
  end

  def update(attrs)
    attrs.each { |k, v| send(:"#{k}=", v) if respond_to?(:"#{k}=") }
    self
  end
end

# vuln-code-snippet start ruby_authz_update_other_user
def update_user(req)
  target_id = req.param('id')
  attrs = { name: req.param('name'), email: req.param('email') }
  User.find(target_id).update(attrs) # vuln-code-snippet vuln-line ruby_authz_update_other_user
  BenchmarkResponse.ok('updated')
end
# vuln-code-snippet end ruby_authz_update_other_user
