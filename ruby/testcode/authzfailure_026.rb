require_relative 'shared'

class CurrentUser
  attr_accessor :id, :role, :name

  def initialize(id, role)
    @id = id
    @role = role
    @name = "user_#{id}"
  end

  def update(attrs)
    attrs.each { |k, v| instance_variable_set(:"@#{k}", v) }
    self
  end
end

# vuln-code-snippet start ruby_authz_role_param
def update_profile(req)
  current_user = CurrentUser.new(req.cookie('user_id'), 'member')
  current_user.update(role: req.param('role')) # vuln-code-snippet vuln-line ruby_authz_role_param
  BenchmarkResponse.ok('profile updated')
end
# vuln-code-snippet end ruby_authz_role_param
